use super::{util, ServiceResolution};
use avahi_sys::{
    avahi_address_snprint, avahi_client_free, avahi_service_browser_free,
    avahi_service_browser_new, avahi_service_resolver_free, avahi_service_resolver_new,
    avahi_simple_poll_free, avahi_simple_poll_loop, AvahiAddress, AvahiBrowserEvent, AvahiClient,
    AvahiClientState, AvahiIfIndex, AvahiLookupResultFlags, AvahiProtocol, AvahiResolverEvent,
    AvahiServiceBrowser, AvahiServiceResolver, AvahiSimplePoll, AvahiStringList,
};
use libc::{c_char, c_int, c_void};
use std::ffi::{CStr, CString};
use std::{mem, ptr};

pub struct AvahiMdnsBrowser {
    #[allow(dead_code)]
    client: *mut AvahiClient,
    poller: *mut AvahiSimplePoll,
    #[allow(dead_code)]
    browser: *mut AvahiServiceBrowser,
    user_data: *mut UserData,
}

struct UserData {
    client: *mut AvahiClient,
    resolver_found_callback: *mut dyn Fn(ServiceResolution),
}

impl AvahiMdnsBrowser {
    pub fn new(
        kind: &str,
        resolver_found_callback: Box<dyn Fn(ServiceResolution)>,
    ) -> Option<Self> {
        let mut err: c_int = 0;

        let poller = util::new_poller()?;
        let client = util::new_client(poller, Some(client_callback), ptr::null_mut(), &mut err)?;

        if err != 0 {
            unsafe { avahi_simple_poll_free(poller) };
            return None;
        }

        let c_kind = CString::new(kind.to_string()).unwrap();

        let user_data = Box::into_raw(Box::new(UserData {
            client,
            resolver_found_callback: Box::into_raw(resolver_found_callback),
        }));

        let browser = unsafe {
            avahi_service_browser_new(
                client,
                util::AVAHI_IF_UNSPEC,
                util::AVAHI_PROTO_UNSPEC,
                c_kind.as_ptr(),
                ptr::null_mut(),
                0,
                Some(browse_callback),
                user_data as *mut c_void,
            )
        };

        if browser == ptr::null_mut() {
            None
        } else {
            Some(Self {
                client,
                poller,
                browser,
                user_data,
            })
        }
    }

    pub fn start(&mut self) {
        unsafe { avahi_simple_poll_loop(self.poller) };
    }
}

impl Drop for AvahiMdnsBrowser {
    fn drop(&mut self) {
        unsafe {
            if self.client != ptr::null_mut() {
                avahi_client_free(self.client);
            }

            if self.poller != ptr::null_mut() {
                avahi_simple_poll_free(self.poller);
            }

            if self.browser != ptr::null_mut() {
                avahi_service_browser_free(self.browser);
            }

            if self.user_data != ptr::null_mut() {
                Box::from_raw(self.user_data);
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
    match event {
        avahi_sys::AvahiBrowserEvent_AVAHI_BROWSER_NEW => {
            let resolver = unsafe {
                avahi_service_resolver_new(
                    (&mut *(userdata as *mut UserData)).client,
                    interface,
                    protocol,
                    name,
                    kind,
                    domain,
                    util::AVAHI_PROTO_UNSPEC,
                    0,
                    Some(resolve_callback),
                    userdata,
                )
            };

            if resolver == ptr::null_mut() {
                panic!("could not create new resolver");
            }
        }
        avahi_sys::AvahiBrowserEvent_AVAHI_BROWSER_FAILURE => panic!("browser failure"),
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
                unsafe { CString::from_vec_unchecked(vec![0; util::AVAHI_ADDRESS_STR_MAX]) };

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
                let user_data = &mut *(userdata as *mut UserData);
                &*(user_data.resolver_found_callback)
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
