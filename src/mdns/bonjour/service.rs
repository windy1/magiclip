use super::backend::{ManagedDNSServiceRef, RegisterServiceParams};
use bonjour_sys::{DNSServiceErrorType, DNSServiceFlags, DNSServiceRef};
use libc::{c_char, c_void};
use std::ffi::CString;
use std::ptr;

const BONJOUR_IF_UNSPEC: u32 = 0;
const BONJOUR_RENAME_FLAGS: DNSServiceFlags = 0;

pub struct MdnsService {
    service: ManagedDNSServiceRef,
    kind: CString,
    port: u16,
}

impl MdnsService {
    pub fn new(kind: &str, port: u16) -> Self {
        Self {
            service: ManagedDNSServiceRef::default(),
            kind: CString::new(kind).unwrap(),
            port,
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        println!("MdnsService#start()\n");

        self.service.register_service(
            RegisterServiceParams::builder()
                .flags(BONJOUR_RENAME_FLAGS)
                .interface_index(BONJOUR_IF_UNSPEC)
                .name(ptr::null())
                .regtype(self.kind.as_ptr())
                .domain(ptr::null())
                .host(ptr::null())
                .port(self.port)
                .txt_len(0)
                .txt_record(ptr::null())
                .callback(Some(register_callback))
                .context(ptr::null_mut())
                .build()?,
        )
    }
}

extern "C" fn register_callback(
    _sd_ref: DNSServiceRef,
    _flags: DNSServiceFlags,
    error: DNSServiceErrorType,
    _name: *const c_char,
    _regtype: *const c_char,
    _domain: *const c_char,
    _context: *mut c_void,
) {
    println!("register_callback()");

    if error != 0 {
        panic!("register_callback reported error (code: {0})", error);
    }

    println!();
}
