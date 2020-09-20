use super::backend::{ManagedDNSServiceRef, RegisterServiceParams};
use super::util;
use crate::mdns::{ServiceRegisteredCallback, ServiceRegistration};
use crate::util::BuilderDelegate;
use bonjour_sys::{DNSServiceErrorType, DNSServiceFlags, DNSServiceRef};
use libc::{c_char, c_void};
use std::ffi::{CStr, CString};
use std::ptr;

const BONJOUR_IF_UNSPEC: u32 = 0;
const BONJOUR_RENAME_FLAGS: DNSServiceFlags = 0;

pub struct MdnsService {
    service: ManagedDNSServiceRef,
    kind: CString,
    port: u16,
    context: *mut BonjourServiceContext,
}

impl MdnsService {
    pub fn new(kind: &str, port: u16) -> Self {
        Self {
            service: ManagedDNSServiceRef::default(),
            kind: CString::new(kind).unwrap(),
            port,
            context: Box::into_raw(Box::default()),
        }
    }

    pub fn set_registered_callback(&mut self, registered_callback: Box<ServiceRegisteredCallback>) {
        unsafe { (*self.context).registered_callback = Some(registered_callback) };
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
                .context(self.context as *mut c_void)
                .build()?,
        )
    }
}

impl Drop for MdnsService {
    fn drop(&mut self) {
        unsafe { Box::from_raw(self.context) };
    }
}

#[derive(Default)]
pub struct BonjourServiceContext {
    registered_callback: Option<Box<ServiceRegisteredCallback>>,
}

impl BonjourServiceContext {
    fn from_raw<'a>(raw: *mut c_void) -> &'a mut Self {
        unsafe { &mut *(raw as *mut BonjourServiceContext) }
    }
}

unsafe extern "C" fn register_callback(
    _sd_ref: DNSServiceRef,
    _flags: DNSServiceFlags,
    error: DNSServiceErrorType,
    name: *const c_char,
    regtype: *const c_char,
    domain: *const c_char,
    context: *mut c_void,
) {
    println!("register_callback()");

    if error != 0 {
        panic!("register_callback reported error (code: {0})", error);
    }

    let domain = util::normalize_domain(CStr::from_ptr(domain).to_str().unwrap());

    let result = ServiceRegistration::builder()
        .name(String::from(CStr::from_ptr(name).to_str().unwrap()))
        .kind(String::from(CStr::from_ptr(regtype).to_str().unwrap()))
        .domain(domain)
        .build()
        .expect("could not build ServiceRegistration");

    println!("result = {:?}\n", result);

    let context = BonjourServiceContext::from_raw(context);

    if let Some(f) = &mut context.registered_callback {
        f(result);
    }
}
