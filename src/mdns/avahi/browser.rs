use super::util;
use avahi_sys::{
    avahi_address_snprint, avahi_service_browser_new, avahi_service_resolver_new,
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
}

impl AvahiMdnsBrowser {
    pub fn new(kind: &str) -> Option<Self> {
        println!("CREATING_BROWSER");

        let mut err: c_int = 0;

        let poller = util::new_poller()?;
        let client = util::new_client(poller, Some(client_callback), ptr::null_mut(), &mut err)?;

        if err != 0 {
            unsafe { avahi_simple_poll_free(poller) };
            return None;
        }

        let c_kind = CString::new(kind.to_string()).unwrap();

        let browser = unsafe {
            avahi_service_browser_new(
                client,
                util::AVAHI_IF_UNSPEC,
                util::AVAHI_PROTO_UNSPEC,
                c_kind.as_ptr(),
                ptr::null_mut(),
                0,
                Some(browse_callback),
                client as *mut c_void,
            )
        };

        if browser == ptr::null_mut() {
            None
        } else {
            Some(Self {
                client,
                poller,
                browser,
            })
        }
    }

    pub fn start(&mut self) {
        println!("STARTING_BROWSER");
        unsafe { avahi_simple_poll_loop(self.poller) };
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
            let (name_r, kind_r, domain_r) = unsafe {
                (
                    CStr::from_ptr(name),
                    CStr::from_ptr(kind),
                    CStr::from_ptr(domain),
                )
            };

            println!(
                "service found: `{}` of type `{}` in domain `{}`",
                name_r.to_str().unwrap(),
                kind_r.to_str().unwrap(),
                domain_r.to_str().unwrap()
            );

            let client = userdata as *mut AvahiClient;

            let resolver = unsafe {
                avahi_service_resolver_new(
                    client,
                    interface,
                    protocol,
                    name,
                    kind,
                    domain,
                    util::AVAHI_PROTO_UNSPEC,
                    0,
                    Some(resolve_callback),
                    client as *mut c_void,
                )
            };

            if resolver == ptr::null_mut() {
                panic!("could not create new resolver");
            }
        }
        _ => {}
    }
}

extern "C" fn resolve_callback(
    _resolver: *mut AvahiServiceResolver,
    _interface: AvahiIfIndex,
    _protocol: AvahiProtocol,
    event: AvahiResolverEvent,
    name: *const ::libc::c_char,
    kind: *const ::libc::c_char,
    domain: *const ::libc::c_char,
    _host_name: *const ::libc::c_char,
    addr: *const AvahiAddress,
    _port: u16,
    _txt: *mut AvahiStringList,
    _flags: AvahiLookupResultFlags,
    _userdata: *mut ::libc::c_void,
) {
    let (name_r, kind_r, domain_r) = unsafe {
        (
            CStr::from_ptr(name),
            CStr::from_ptr(kind),
            CStr::from_ptr(domain),
        )
    };

    match event {
        avahi_sys::AvahiResolverEvent_AVAHI_RESOLVER_FAILURE => println!(
            "failed to resolve service `{}` of type `{}` in domain `{}`",
            name_r.to_str().unwrap(),
            kind_r.to_str().unwrap(),
            domain_r.to_str().unwrap()
        ),
        avahi_sys::AvahiResolverEvent_AVAHI_RESOLVER_FOUND => {
            println!(
                "Service `{}` of type `{}` in domain `{}`:",
                name_r.to_str().unwrap(),
                kind_r.to_str().unwrap(),
                domain_r.to_str().unwrap()
            );

            let address =
                unsafe { CString::from_vec_unchecked(vec![0; util::AVAHI_ADDRESS_STR_MAX]) };

            unsafe {
                avahi_address_snprint(
                    address.as_ptr() as *mut c_char,
                    mem::size_of_val(&address),
                    addr,
                )
            };

            println!("address = {}", address.into_string().unwrap());
        }
        _ => {}
    }
}

extern "C" fn client_callback(
    _client: *mut AvahiClient,
    _state: AvahiClientState,
    _userdata: *mut c_void,
) {
}
