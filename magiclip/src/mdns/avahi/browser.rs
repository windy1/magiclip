use super::address;
use super::raw_browser::{ManagedAvahiServiceBrowser, ManagedAvahiServiceBrowserParams};
use crate::builder::BuilderDelegate;
use crate::ffi::{cstr, FromRaw};
use crate::mdns::client::{ManagedAvahiClient, ManagedAvahiClientParams};
use crate::mdns::constants;
use crate::mdns::poll::ManagedAvahiSimplePoll;
use crate::mdns::resolver::{
    ManagedAvahiServiceResolver, ManagedAvahiServiceResolverParams, ServiceResolverSet,
};
use crate::mdns::{ResolverFoundCallback, ServiceResolution};
use avahi_sys::{
    AvahiAddress, AvahiBrowserEvent, AvahiClient, AvahiClientFlags, AvahiClientState, AvahiIfIndex,
    AvahiLookupResultFlags, AvahiProtocol, AvahiResolverEvent, AvahiServiceBrowser,
    AvahiServiceResolver, AvahiStringList,
};
use libc::{c_char, c_void};
use std::ffi::CString;
use std::{fmt, ptr};

#[derive(Debug)]
pub struct MdnsBrowser {
    poll: Option<ManagedAvahiSimplePoll>,
    browser: Option<ManagedAvahiServiceBrowser>,
    kind: CString,
    context: *mut AvahiBrowserContext,
}

impl MdnsBrowser {
    pub fn new(kind: &str) -> Self {
        Self {
            poll: None,
            browser: None,
            kind: CString::new(kind.to_string()).unwrap(),
            context: Box::into_raw(Box::default()),
        }
    }

    pub fn set_resolver_found_callback(
        &mut self,
        resolver_found_callback: Box<ResolverFoundCallback>,
    ) {
        unsafe { (*self.context).resolver_found_callback = Some(resolver_found_callback) };
    }

    pub fn start(&mut self) -> Result<(), String> {
        debug!("Browsing services: {:?}", self);

        self.poll = Some(ManagedAvahiSimplePoll::new()?);

        let client = ManagedAvahiClient::new(
            ManagedAvahiClientParams::builder()
                .poll(self.poll.as_ref().unwrap())
                .flags(AvahiClientFlags(0))
                .callback(Some(client_callback))
                .userdata(ptr::null_mut())
                .build()?,
        )?;

        unsafe {
            (*self.context).client = Some(client);

            self.browser = Some(ManagedAvahiServiceBrowser::new(
                ManagedAvahiServiceBrowserParams::builder()
                    .client(&(*self.context).client.as_ref().unwrap())
                    .interface(constants::AVAHI_IF_UNSPEC)
                    .protocol(constants::AVAHI_PROTO_UNSPEC)
                    .kind(self.kind.as_ptr())
                    .domain(ptr::null_mut())
                    .flags(0)
                    .callback(Some(browse_callback))
                    .userdata(self.context as *mut c_void)
                    .build()?,
            )?);
        }

        self.poll.as_ref().unwrap().start_loop()
    }
}

impl Drop for MdnsBrowser {
    fn drop(&mut self) {
        unsafe {
            Box::from_raw(self.context);
        }
    }
}

#[derive(FromRaw)]
struct AvahiBrowserContext {
    client: Option<ManagedAvahiClient>,
    resolvers: ServiceResolverSet,
    resolver_found_callback: Option<Box<ResolverFoundCallback>>,
}

impl Default for AvahiBrowserContext {
    fn default() -> Self {
        AvahiBrowserContext {
            client: None,
            resolvers: ServiceResolverSet::default(),
            resolver_found_callback: None,
        }
    }
}

impl fmt::Debug for AvahiBrowserContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("AvahiBrowserContext")
            .field("client", &self.client)
            .field("resolvers", &self.resolvers)
            .finish()
    }
}

unsafe extern "C" fn browse_callback(
    _browser: *mut AvahiServiceBrowser,
    interface: AvahiIfIndex,
    protocol: AvahiProtocol,
    event: AvahiBrowserEvent,
    name: *const c_char,
    kind: *const c_char,
    domain: *const c_char,
    _flags: AvahiLookupResultFlags,
    userdata: *mut c_void,
) {
    let context = AvahiBrowserContext::from_raw(userdata);

    debug!("browse_callback()");
    debug!("\tcontext = {:?}", context);

    match event {
        avahi_sys::AvahiBrowserEvent_AVAHI_BROWSER_NEW => {
            context.resolvers.insert(
                ManagedAvahiServiceResolver::new(
                    ManagedAvahiServiceResolverParams::builder()
                        .client(context.client.as_ref().unwrap())
                        .interface(interface)
                        .protocol(protocol)
                        .name(name)
                        .kind(kind)
                        .domain(domain)
                        .aprotocol(constants::AVAHI_PROTO_UNSPEC)
                        .flags(0)
                        .callback(Some(resolve_callback))
                        .userdata(userdata)
                        .build()
                        .unwrap(),
                )
                .unwrap(),
            );

            // context.resolver = Some(
            //     ManagedAvahiServiceResolver::new(
            //         ManagedAvahiServiceResolverParams::builder()
            //             .client(context.client.as_ref().unwrap())
            //             .interface(interface)
            //             .protocol(protocol)
            //             .name(name)
            //             .kind(kind)
            //             .domain(domain)
            //             .aprotocol(constants::AVAHI_PROTO_UNSPEC)
            //             .flags(0)
            //             .callback(Some(resolve_callback))
            //             .userdata(userdata)
            //             .build()
            //             .unwrap(),
            //     )
            //     .unwrap(),
            // );
        }
        avahi_sys::AvahiBrowserEvent_AVAHI_BROWSER_FAILURE => panic!("browser failure"),
        _ => {}
    };
}

unsafe extern "C" fn resolve_callback(
    resolver: *mut AvahiServiceResolver,
    _interface: AvahiIfIndex,
    _protocol: AvahiProtocol,
    event: AvahiResolverEvent,
    name: *const c_char,
    kind: *const c_char,
    domain: *const c_char,
    host_name: *const c_char,
    addr: *const AvahiAddress,
    port: u16,
    _txt: *mut AvahiStringList,
    _flags: AvahiLookupResultFlags,
    userdata: *mut c_void,
) {
    debug!("resolve_callback()");

    let name = cstr::raw_to_str(name);
    let kind = cstr::raw_to_str(kind);
    let domain = cstr::raw_to_str(domain);

    let context = AvahiBrowserContext::from_raw(userdata);

    match event {
        avahi_sys::AvahiResolverEvent_AVAHI_RESOLVER_FAILURE => warn!(
            "failed to resolve service `{}` of type `{}` in domain `{}`",
            name, kind, domain
        ),
        avahi_sys::AvahiResolverEvent_AVAHI_RESOLVER_FOUND => {
            let host_name = cstr::raw_to_str(host_name);
            let address = address::get_ip(addr);

            let result = ServiceResolution::builder()
                .name(name.to_string())
                .kind(kind.to_string())
                .domain(domain.to_string())
                .host_name(host_name.to_string())
                .address(address)
                .port(port)
                .build()
                .unwrap();

            debug!("Service resolved: {:?}", result);

            if let Some(f) = &context.resolver_found_callback {
                f(result);
            } else {
                warn!("Service resolved but no callback was set");
            }
        }
        _ => {}
    };

    context.resolvers.remove_raw(resolver);
}

extern "C" fn client_callback(
    _client: *mut AvahiClient,
    state: AvahiClientState,
    _userdata: *mut c_void,
) {
    debug!("client_callback()");

    match state {
        avahi_sys::AvahiClientState_AVAHI_CLIENT_FAILURE => panic!("client failure"),
        _ => {}
    }
}
