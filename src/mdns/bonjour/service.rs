use super::backend::{ManagedDNSServiceRef, RegisterServiceParams};
use crate::mdns::err::{ErrorCallback, HandleError};
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
    context: *mut BonjourServiceContext,
}

struct BonjourServiceContext {
    error_callback: Option<Box<ErrorCallback>>,
}

impl MdnsService {
    pub fn new(kind: &str, port: u16) -> Self {
        Self {
            service: ManagedDNSServiceRef::new(),
            kind: CString::new(kind).unwrap(),
            port,
            context: Box::into_raw(Box::new(BonjourServiceContext {
                error_callback: None,
            })),
        }
    }

    pub fn set_error_callback(&mut self, error_callback: Box<ErrorCallback>) {
        unsafe { (*self.context).error_callback = Some(error_callback) };
    }

    pub fn start(&mut self) -> Result<(), String> {
        println!("registering service");

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

impl Drop for MdnsService {
    fn drop(&mut self) {
        if self.context != ptr::null_mut() {
            Box::from(self.context);
        }
    }
}

impl HandleError for BonjourServiceContext {
    fn error_callback(&self) -> Option<&ErrorCallback> {
        match self.error_callback {
            Some(ref f) => Some(f),
            None => None,
        }
    }
}

extern "C" fn register_callback(
    _sd_ref: DNSServiceRef,
    _flags: DNSServiceFlags,
    error: DNSServiceErrorType,
    _name: *const c_char,
    _regtype: *const c_char,
    _domain: *const c_char,
    context: *mut c_void,
) {
    let context = unsafe { &mut *(context as *mut BonjourServiceContext) };
    println!("register_callback");
    if error != 0 {
        context.handle_error(&format!(
            "register_callback reported error (code: {0})",
            error
        ));
    }
}
