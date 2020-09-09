use bonjour_sys::{
    kDNSServiceProperty_DaemonVersion, DNSServiceCreateConnection, DNSServiceErrorType,
    DNSServiceFlags, DNSServiceGetProperty, DNSServiceProcessResult, DNSServiceRef,
    DNSServiceRegister, DNSServiceRegisterReply,
};
use libc::{c_char, c_void};
use std::ffi::CString;
use std::{mem, ptr};

const BOUNJOUR_IF_UNSPEC: u32 = 0;
const BONJOUR_RENAME_FLAGS: DNSServiceFlags = 0;

pub struct MdnsService {
    service: DNSServiceRef,
    name: CString,
    kind: CString,
    port: u16,
}

impl MdnsService {
    pub fn new(name: &str, kind: &str, port: u16) -> Option<Self> {
        Some(Self {
            service: ptr::null_mut(),
            name: CString::new(name).unwrap(),
            kind: CString::new(kind).unwrap(),
            port,
        })
    }

    pub fn start(&mut self) {
        println!("registering service");

        let err = unsafe {
            DNSServiceRegister(
                &mut self.service as *mut DNSServiceRef, // sdRef
                BONJOUR_RENAME_FLAGS,                    // flags
                BOUNJOUR_IF_UNSPEC,                      // interfaceIndex
                ptr::null(),                             // name
                self.kind.as_ptr(),                      // regtype
                ptr::null(),                             // domain
                ptr::null(),                             // host
                self.port,                               // port
                0,                                       // txtLen
                ptr::null(),                             // txtRecord
                Some(register_callback),                 // callback
                ptr::null_mut(),                         // context
            )
        };

        if (err != 0) {
            panic!("could not register service with error code: `{0}`", err);
        }

        let err = unsafe { DNSServiceProcessResult(self.service) };

        if err != 0 {
            panic!("could not start processing loop: `{0}`", err);
        }
    }
}

extern "C" fn register_callback(
    _sd_ref: DNSServiceRef,
    _flags: DNSServiceFlags,
    _error_code: DNSServiceErrorType,
    _name: *const c_char,
    _regtype: *const c_char,
    _domain: *const c_char,
    _context: *mut c_void,
) {
    println!("register_callback");
}
