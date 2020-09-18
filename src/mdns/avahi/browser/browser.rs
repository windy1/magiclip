use super::backend::AvahiServiceBrowserParams;
use crate::mdns::client::AvahiClientParams;
use crate::mdns::constants;
use crate::mdns::err::{ErrorCallback, HandleError};
use crate::mdns::poll;
use crate::mdns::ServiceResolution;
use avahi_sys::{
    avahi_address_snprint, avahi_client_free, avahi_service_browser_free,
    avahi_service_resolver_free, avahi_service_resolver_new, avahi_simple_poll_free,
    avahi_simple_poll_loop, AvahiAddress, AvahiBrowserEvent, AvahiClient, AvahiClientState,
    AvahiIfIndex, AvahiLookupResultFlags, AvahiProtocol, AvahiResolverEvent, AvahiServiceBrowser,
    AvahiServiceResolver, AvahiSimplePoll, AvahiStringList,
};
use libc::{c_char, c_void};
use std::convert::TryInto;
use std::ffi::{CStr, CString};
use std::{mem, ptr};

pub type ResolverFoundCallback = dyn Fn(ServiceResolution);

pub struct MdnsBrowser {
    poller: *mut AvahiSimplePoll,
    browser: *mut AvahiServiceBrowser,
    kind: CString,
    context: *mut AvahiBrowserContext,
}

struct AvahiBrowserContext {
    client: *mut AvahiClient,
    resolver_found_callback: Box<ResolverFoundCallback>,
    error_callback: Option<Box<ErrorCallback>>,
}

impl MdnsBrowser {
    pub fn new(kind: &str, resolver_found_callback: Box<dyn Fn(ServiceResolution)>) -> Self {
        Self {
            poller: ptr::null_mut(),
            browser: ptr::null_mut(),
            kind: CString::new(kind.to_string()).unwrap(),
            context: Box::into_raw(Box::new(AvahiBrowserContext {
                client: ptr::null_mut(),
                resolver_found_callback,
                error_callback: None,
            })),
        }
    }

    pub fn set_error_callback(&mut self, error_callback: Box<ErrorCallback>) {
        unsafe { (*self.context).error_callback = Some(error_callback) };
    }

    pub fn start(&mut self) -> Result<(), String> {
        self.poller = poll::new_poller()?;

        unsafe {
            (*self.context).client = AvahiClientParams::builder()
                .poller(self.poller)
                .callback(Some(client_callback))
                .context(ptr::null_mut())
                .build()?
                .try_into()?;
        };

        self.browser = unsafe {
            AvahiServiceBrowserParams::builder()
                .client((*self.context).client)
                .interface(constants::AVAHI_IF_UNSPEC)
                .protocol(constants::AVAHI_PROTO_UNSPEC)
                .kind(self.kind.as_ptr())
                .domain(ptr::null_mut())
                .flags(0)
                .callback(Some(browse_callback))
                .context(self.context as *mut c_void)
                .build()?
                .try_into()?
        };

        unsafe { avahi_simple_poll_loop(self.poller) };

        Ok(())
    }
}

impl Drop for MdnsBrowser {
    fn drop(&mut self) {
        unsafe {
            if self.poller != ptr::null_mut() {
                avahi_simple_poll_free(self.poller);
            }

            if self.browser != ptr::null_mut() {
                avahi_service_browser_free(self.browser);
            }

            Box::from_raw(self.context);
        }
    }
}

impl HandleError for AvahiBrowserContext {
    fn error_callback(&self) -> Option<&Box<ErrorCallback>> {
        self.error_callback.as_ref()
    }
}

impl Drop for AvahiBrowserContext {
    fn drop(&mut self) {
        unsafe {
            if self.client != ptr::null_mut() {
                avahi_client_free(self.client);
            }
        }
    }
}

extern "C" fn browse_callback(
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
    let context = unsafe { &mut *(userdata as *mut AvahiBrowserContext) };
    match event {
        avahi_sys::AvahiBrowserEvent_AVAHI_BROWSER_NEW => {
            let resolver = unsafe {
                avahi_service_resolver_new(
                    context.client,
                    interface,
                    protocol,
                    name,
                    kind,
                    domain,
                    constants::AVAHI_PROTO_UNSPEC,
                    0,
                    Some(resolve_callback),
                    userdata,
                )
            };

            if resolver == ptr::null_mut() {
                context.handle_error("could not create new resolver");
            }
        }
        avahi_sys::AvahiBrowserEvent_AVAHI_BROWSER_FAILURE => {
            context.handle_error("browser failure")
        }
        _ => {}
    }
}

extern "C" fn resolve_callback(
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
    let (name_r, kind_r, domain_r, host_name_r) = unsafe {
        (
            CStr::from_ptr(name).to_str().unwrap(),
            CStr::from_ptr(kind).to_str().unwrap(),
            CStr::from_ptr(domain).to_str().unwrap(),
            CStr::from_ptr(host_name).to_str().unwrap(),
        )
    };

    match event {
        avahi_sys::AvahiResolverEvent_AVAHI_RESOLVER_FAILURE => println!(
            "failed to resolve service `{}` of type `{}` in domain `{}`",
            name_r, kind_r, domain_r
        ),
        avahi_sys::AvahiResolverEvent_AVAHI_RESOLVER_FOUND => {
            let address =
                unsafe { CString::from_vec_unchecked(vec![0; constants::AVAHI_ADDRESS_STR_MAX]) };

            unsafe {
                avahi_address_snprint(
                    address.as_ptr() as *mut c_char,
                    mem::size_of_val(&address),
                    addr,
                )
            };

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

            let callback = unsafe {
                let context = &mut *(userdata as *mut AvahiBrowserContext);
                &*(context.resolver_found_callback)
            };

            callback(result);
        }
        _ => {}
    }

    unsafe { avahi_service_resolver_free(resolver) };
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
