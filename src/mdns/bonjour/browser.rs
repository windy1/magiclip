use super::backend::{BrowseServicesParams, ManagedDNSServiceRef, ServiceResolveParams};
use crate::mdns::err::{ErrorCallback, HandleError};
use crate::mdns::{ResolverFoundCallback, ServiceResolution};
use bonjour_sys::{DNSServiceErrorType, DNSServiceFlags, DNSServiceRef};
use libc::{c_char, c_uchar, c_void};
use std::ffi::{CStr, CString};
use std::ptr;

pub struct MdnsBrowser {
    service: ManagedDNSServiceRef,
    kind: CString,
    context: *mut BonjourBrowserContext,
}

struct BonjourBrowserContext {
    error_callback: Option<Box<ErrorCallback>>,
    resolver_found_callback: Box<ResolverFoundCallback>,
}

impl MdnsBrowser {
    pub fn new(kind: &str, resolver_found_callback: Box<dyn Fn(ServiceResolution)>) -> Self {
        Self {
            service: ManagedDNSServiceRef::new(),
            kind: CString::new(kind).unwrap(),
            context: Box::into_raw(Box::new(BonjourBrowserContext {
                error_callback: None,
                resolver_found_callback,
            })),
        }
    }

    pub fn set_error_callback(&self, error_callback: Box<ErrorCallback>) {
        unsafe { (*self.context).error_callback = Some(error_callback) };
    }

    pub fn start(&mut self) -> Result<(), String> {
        println!("starting browser");

        self.service.browse_services(
            BrowseServicesParams::builder()
                .flags(0)
                .interface_index(0)
                .regtype(self.kind.as_ptr())
                .domain(ptr::null_mut())
                .callback(Some(browse_callback))
                .context(self.context as *mut c_void)
                .build()?,
        )
    }
}

impl Drop for MdnsBrowser {
    fn drop(&mut self) {
        if self.context != ptr::null_mut() {
            Box::from(self.context);
        }
    }
}

impl HandleError for BonjourBrowserContext {
    fn error_callback(&self) -> Option<&Box<ErrorCallback>> {
        self.error_callback.as_ref()
    }
}

extern "C" fn browse_callback(
    _sd_ref: DNSServiceRef,
    _flags: DNSServiceFlags,
    interface_index: u32,
    error: DNSServiceErrorType,
    name: *const c_char,
    regtype: *const c_char,
    domain: *const c_char,
    context: *mut c_void,
) {
    let (name_r, regtype_r, domain_r) = unsafe {
        (
            CStr::from_ptr(name).to_str().unwrap(),
            CStr::from_ptr(regtype).to_str().unwrap(),
            CStr::from_ptr(domain).to_str().unwrap(),
        )
    };

    println!("browse_callback()");
    println!("name = {:?}", name_r);
    println!("regtype = {:?}", regtype_r);
    println!("domain = {:?}", domain_r);

    let ctx = unsafe { &mut *(context as *mut BonjourBrowserContext) };

    if error != 0 {
        ctx.handle_error(&format!("browse_callback reported error (code: {})", error));
        return;
    }

    let result = ManagedDNSServiceRef::new().resolve_service(
        ServiceResolveParams::builder()
            .flags(0)
            .interface_index(interface_index)
            .name(name)
            .regtype(regtype)
            .domain(domain)
            .callback(Some(resolve_callback))
            .context(context)
            .build()
            .expect("could not build ServiceResolveParams"),
    );

    if let Err(err) = result {
        ctx.handle_error(&err);
    }
}

extern "C" fn resolve_callback(
    _sd_ref: DNSServiceRef,
    _flags: DNSServiceFlags,
    _interface_index: u32,
    error: DNSServiceErrorType,
    fullname: *const c_char,
    host_target: *const c_char,
    _port: u16,
    _txt_len: u16,
    _txt_record: *const c_uchar,
    context: *mut c_void,
) {
    let (fullname_r, host_target_r) = unsafe {
        (
            CStr::from_ptr(fullname).to_str().unwrap(),
            CStr::from_ptr(host_target),
        )
    };

    println!("fullname = {:?}", fullname_r);
    println!("host_target = {:?}", host_target_r);

    let context = unsafe { &mut *(context as *mut BonjourBrowserContext) };

    if error != 0 {
        context.handle_error(&format!(
            "error reported by resolve_callback: (code: {})",
            error
        ));
    }
}
