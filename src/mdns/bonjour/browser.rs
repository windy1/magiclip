use crate::mdns::ServiceResolution;
use bonjour_sys::{
    DNSServiceBrowse, DNSServiceErrorType, DNSServiceFlags, DNSServiceProcessResult, DNSServiceRef,
};
use libc::{c_char, c_void};
use std::ffi::CString;
use std::ptr;

pub struct MdnsBrowser {
    service: DNSServiceRef,
    kind: CString,
    resolver_found_callback: Box<dyn Fn(ServiceResolution)>,
}

impl MdnsBrowser {
    pub fn new(
        kind: &str,
        resolver_found_callback: Box<dyn Fn(ServiceResolution)>,
    ) -> Option<Self> {
        Some(Self {
            service: ptr::null_mut(),
            kind: CString::new(kind).unwrap(),
            resolver_found_callback,
        })
    }

    pub fn start(&mut self) {
        println!("starting browser");

        let err = unsafe {
            DNSServiceBrowse(
                &mut self.service as *mut DNSServiceRef, // sdRef
                0,                                       // flags
                0,                                       // interfaceIndex
                self.kind.as_ptr(),                      // regtype
                ptr::null_mut(),                         // domain
                Some(browse_callback),                   // callback
                ptr::null_mut(),                         // context
            )
        };

        if err != 0 {
            panic!("could not browse services with error code: `{0}`", err);
        }

        let err = unsafe { DNSServiceProcessResult(self.service) };

        if err != 0 {
            panic!("could not start processing loop: `{0}`", err);
        }
    }
}

extern "C" fn browse_callback(
    _sd_ref: DNSServiceRef,
    _flags: DNSServiceFlags,
    _interface_index: u32,
    _error_code: DNSServiceErrorType,
    _name: *const c_char,
    _regtype: *const c_char,
    _domain: *const c_char,
    _context: *mut c_void,
) {
    println!("browse_callback");
}
