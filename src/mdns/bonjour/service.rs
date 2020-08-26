use bonjour_sys::{
    DNSServiceErrorType, DNSServiceFlags, DNSServiceRef, DNSServiceRegister,
    DNSServiceRegisterReply,
};
use std::ffi::CString;
use std::os::raw::{c_char, c_void};
use std::ptr;

const BOUNJOUR_IF_UNSPEC: u32 = 0;
const BONJOUR_RENAME_FLAGS: DNSServiceFlags = 0;

pub struct MdnsService {
    service: *mut DNSServiceRef,
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

    pub fn start(&self) {
        println!("registering service");

        println!("name = {:?}", self.name);
        println!("kind = {:?}", self.kind);

        let err = unsafe {
            DNSServiceRegister(
                self.service,
                BONJOUR_RENAME_FLAGS,
                BOUNJOUR_IF_UNSPEC,
                self.name.as_ptr(),
                self.kind.as_ptr(),
                ptr::null_mut(),
                ptr::null_mut(),
                self.port,
                0,
                ptr::null_mut(),
                Some(register_callback),
                ptr::null_mut(),
            )
        };

        println!("err = {}", err);
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
