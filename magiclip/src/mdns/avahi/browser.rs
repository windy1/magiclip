use super::raw_browser::{ManagedAvahiServiceBrowser, ManagedAvahiServiceBrowserParams};
use crate::builder::BuilderDelegate;
use crate::mdns::client::{ManagedAvahiClient, ManagedAvahiClientParams};
use crate::mdns::constants;
use crate::mdns::poll::ManagedAvahiSimplePoll;
use crate::mdns::resolver::{ManagedAvahiServiceResolver, ManagedAvahiServiceResolverParams};
use crate::mdns::{ResolverFoundCallback, ServiceResolution};
use avahi_sys::{
    avahi_address_snprint, AvahiAddress, AvahiBrowserEvent, AvahiClient, AvahiClientFlags,
    AvahiClientState, AvahiIfIndex, AvahiLookupResultFlags, AvahiProtocol, AvahiResolverEvent,
    AvahiServiceBrowser, AvahiServiceResolver, AvahiStringList,
};
use libc::{c_char, c_void};
use std::ffi::{CStr, CString};
use std::{mem, ptr};

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

struct AvahiBrowserContext {
    client: Option<ManagedAvahiClient>,
    resolver: Option<ManagedAvahiServiceResolver>,
    resolver_found_callback: Option<Box<ResolverFoundCallback>>,
}

impl Default for AvahiBrowserContext {
    fn default() -> Self {
        AvahiBrowserContext {
            client: None,
            resolver: None,
            resolver_found_callback: None,
        }
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
    let mut context = &mut *(userdata as *mut AvahiBrowserContext);

    match event {
        avahi_sys::AvahiBrowserEvent_AVAHI_BROWSER_NEW => {
            context.resolver = Some(
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
        }
        avahi_sys::AvahiBrowserEvent_AVAHI_BROWSER_FAILURE => panic!("browser failure"),
        _ => {}
    };
}

unsafe extern "C" fn resolve_callback(
    _resolver: *mut AvahiServiceResolver,
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
    let name_r = CStr::from_ptr(name).to_str().unwrap();
    let kind_r = CStr::from_ptr(kind).to_str().unwrap();
    let domain_r = CStr::from_ptr(domain).to_str().unwrap();
    let host_name_r = CStr::from_ptr(host_name).to_str().unwrap();

    match event {
        avahi_sys::AvahiResolverEvent_AVAHI_RESOLVER_FAILURE => warn!(
            "failed to resolve service `{}` of type `{}` in domain `{}`",
            name_r, kind_r, domain_r
        ),
        avahi_sys::AvahiResolverEvent_AVAHI_RESOLVER_FOUND => {
            let address = CString::from_vec_unchecked(vec![0; constants::AVAHI_ADDRESS_STR_MAX]);

            avahi_address_snprint(
                address.as_ptr() as *mut c_char,
                mem::size_of_val(&address),
                addr,
            );

            let address = address
                .into_string()
                .unwrap()
                .trim_matches(char::from(0))
                .to_string();

            let result = ServiceResolution::builder()
                .name(name_r.to_string())
                .kind(kind_r.to_string())
                .domain(domain_r.to_string())
                .host_name(host_name_r.to_string())
                .address(address)
                .port(port)
                .build()
                .unwrap();

            let context = &mut *(userdata as *mut AvahiBrowserContext);

            if let Some(f) = &context.resolver_found_callback {
                f(result);
            }
        }
        _ => {}
    }
}

extern "C" fn client_callback(
    _client: *mut AvahiClient,
    state: AvahiClientState,
    _userdata: *mut c_void,
) {
    match state {
        avahi_sys::AvahiClientState_AVAHI_CLIENT_FAILURE => panic!("client failure"),
        _ => {}
    }
}
