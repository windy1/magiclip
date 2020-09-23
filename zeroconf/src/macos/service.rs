use super::compat;
use super::service_ref::{ManagedDNSServiceRef, RegisterServiceParams};
use crate::builder::BuilderDelegate;
use crate::ffi::{cstr, FromRaw};
use crate::{ServiceRegisteredCallback, ServiceRegistration};
use bonjour_sys::{DNSServiceErrorType, DNSServiceFlags, DNSServiceRef};
use libc::{c_char, c_void};
use std::any::Any;
use std::ffi::CString;
use std::ptr;
use std::sync::Arc;

const BONJOUR_IF_UNSPEC: u32 = 0;
const BONJOUR_RENAME_FLAGS: DNSServiceFlags = 0;

#[derive(Debug)]
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

    pub fn set_context(&mut self, context: Box<dyn Any>) {
        unsafe { (*self.context).user_context = Some(Arc::from(context)) };
    }

    pub fn start(&mut self) -> Result<(), String> {
        debug!("Registering service: {:?}", self);

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

#[derive(Default, FromRaw)]
struct BonjourServiceContext {
    registered_callback: Option<Box<ServiceRegisteredCallback>>,
    user_context: Option<Arc<dyn Any>>,
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
    if error != 0 {
        panic!("register_callback() reported error (code: {0})", error);
    }

    let domain = compat::normalize_domain(cstr::raw_to_str(domain));

    let result = ServiceRegistration::builder()
        .name(cstr::copy_raw(name))
        .kind(cstr::copy_raw(regtype))
        .domain(domain)
        .build()
        .expect("could not build ServiceRegistration");

    let context = BonjourServiceContext::from_raw(context);

    if let Some(f) = &mut context.registered_callback {
        f(result, context.user_context.clone());
    } else {
        warn!("Service registered but no callback has been set");
    }
}
