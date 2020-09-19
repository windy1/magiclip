use super::resolver::{MdnsServiceResolver, ServiceResolveParams};
use crate::mdns::err::{ErrorCallback, HandleError};
use crate::mdns::{ResolverFoundCallback, ServiceResolution};
use bonjour_sys::{
    DNSServiceBrowse, DNSServiceErrorType, DNSServiceFlags, DNSServiceProcessResult, DNSServiceRef,
    DNSServiceRefDeallocate, DNSServiceResolve,
};
use libc::{c_char, c_uchar, c_void};
use std::ffi::{CStr, CString};
use std::ptr;

pub struct MdnsBrowser {
    service: DNSServiceRef,
    kind: CString,
    resolver_found_callback: Box<ResolverFoundCallback>,
    context: *mut BonjourBrowserContext,
}

struct BonjourBrowserContext {
    error_callback: Option<Box<ErrorCallback>>,
    resolve_service: DNSServiceRef,
}

impl MdnsBrowser {
    pub fn new(kind: &str, resolver_found_callback: Box<dyn Fn(ServiceResolution)>) -> Self {
        Self {
            service: ptr::null_mut(),
            kind: CString::new(kind).unwrap(),
            resolver_found_callback,
            context: Box::into_raw(Box::new(BonjourBrowserContext {
                error_callback: None,
                resolve_service: ptr::null_mut(),
            })),
        }
    }

    pub fn set_error_callback(&self, error_callback: Box<ErrorCallback>) {
        unsafe { (*self.context).error_callback = Some(error_callback) };
    }

    pub fn start(&mut self) -> Result<(), String> {
        println!("starting browser");

        let err = unsafe {
            DNSServiceBrowse(
                &mut self.service as *mut DNSServiceRef, // sdRef
                0,                                       // flags
                0,                                       // interfaceIndex
                self.kind.as_ptr(),                      // regtype
                ptr::null_mut(),                         // domain
                Some(browse_callback),                   // callback
                self.context as *mut c_void,             // context
            )
        };

        if err != 0 {
            return Err(
                format!("could not browse services with error code: `{0}`", err).to_string(),
            );
        }

        loop {
            let err = unsafe { DNSServiceProcessResult(self.service) };

            if err != 0 {
                return Err(format!("could not start processing loop: `{}`", err).to_string());
            }
        }
    }
}

impl Drop for MdnsBrowser {
    fn drop(&mut self) {
        unsafe {
            if self.service != ptr::null_mut() {
                DNSServiceRefDeallocate(self.service);
            }

            if self.context != ptr::null_mut() {
                Box::from(self.context);
            }
        }
    }
}

impl HandleError for BonjourBrowserContext {
    fn error_callback(&self) -> Option<&Box<ErrorCallback>> {
        self.error_callback.as_ref()
    }
}

impl Drop for BonjourBrowserContext {
    fn drop(&mut self) {
        unsafe {
            if self.resolve_service != ptr::null_mut() {
                DNSServiceRefDeallocate(self.resolve_service);
            }
        }
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

    let result = MdnsServiceResolver::new().resolve(
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
    sdRef: DNSServiceRef,
    flags: DNSServiceFlags,
    interface_index: u32,
    error: DNSServiceErrorType,
    fullname: *const c_char,
    host_target: *const c_char,
    port: u16,
    txt_len: u16,
    txt_record: *const c_uchar,
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
}
