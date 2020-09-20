use super::backend::{
    BrowseServicesParams, GetAddressInfoParams, ManagedDNSServiceRef, ServiceResolveParams,
};
use super::util;
use crate::mdns::{ResolverFoundCallback, ServiceResolution};
use crate::util::{cstr, BuilderDelegate, CloneRaw, FromRaw};
use bonjour_sys::{sockaddr, DNSServiceErrorType, DNSServiceFlags, DNSServiceRef};
use libc::{c_char, c_uchar, c_void, in_addr, sockaddr_in};
use std::ffi::{CStr, CString};
use std::fmt::{self, Formatter};
use std::ptr;
use std::sync::Arc;

pub struct MdnsBrowser {
    service: ManagedDNSServiceRef,
    kind: CString,
    context: *mut BonjourBrowserContext,
}

impl MdnsBrowser {
    pub fn new(kind: &str) -> Self {
        Self {
            service: ManagedDNSServiceRef::default(),
            kind: CString::new(kind).unwrap(),
            context: Box::into_raw(Box::default()),
        }
    }

    pub fn set_resolver_found_callback(
        &self,
        resolver_found_callback: Box<dyn Fn(ServiceResolution)>,
    ) {
        unsafe {
            (*self.context).resolver_found_callback = Some(Arc::from(resolver_found_callback))
        };
    }

    pub fn start(&mut self) -> Result<(), String> {
        println!("MdnsBrowser#start()\n");

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
        unsafe { Box::from_raw(self.context) };
    }
}

#[derive(Default, Clone, FromRaw, CloneRaw)]
struct BonjourBrowserContext {
    resolver_found_callback: Option<Arc<ResolverFoundCallback>>,
    resolved_name: Option<String>,
    resolved_kind: Option<String>,
    resolved_domain: Option<String>,
    resolved_port: u16,
}

impl fmt::Debug for BonjourBrowserContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("BonjourResolverContext")
            .field("resolved_name", &self.resolved_name)
            .field("resolved_kind", &self.resolved_kind)
            .field("resolved_domain", &self.resolved_domain)
            .field("resolved_port", &self.resolved_port)
            .finish()
    }
}

impl Drop for BonjourBrowserContext {
    fn drop(&mut self) {
        println!("BonjourResolverContext#drop()");
    }
}

unsafe extern "C" fn browse_callback(
    _sd_ref: DNSServiceRef,
    _flags: DNSServiceFlags,
    interface_index: u32,
    error: DNSServiceErrorType,
    name: *const c_char,
    regtype: *const c_char,
    domain: *const c_char,
    context: *mut c_void,
) {
    println!("browse_callback()");

    let mut ctx = BonjourBrowserContext::clone_raw(context);

    if error != 0 {
        panic!("browse_callback() reported error (code: {})", error);
    }

    ctx.resolved_name = Some(String::from(cstr::raw_to_str(name)));
    ctx.resolved_kind = Some(String::from(cstr::raw_to_str(regtype)));
    ctx.resolved_domain = Some(String::from(cstr::raw_to_str(domain)));

    println!("context = {:?}\n", ctx);

    ManagedDNSServiceRef::default()
        .resolve_service(
            ServiceResolveParams::builder()
                .flags(bonjour_sys::kDNSServiceFlagsForceMulticast)
                .interface_index(interface_index)
                .name(name)
                .regtype(regtype)
                .domain(domain)
                .callback(Some(resolve_callback))
                .context(Box::into_raw(ctx) as *mut c_void)
                .build()
                .expect("could not build ServiceResolveParams"),
        )
        .unwrap();
}

unsafe extern "C" fn resolve_callback(
    _sd_ref: DNSServiceRef,
    _flags: DNSServiceFlags,
    interface_index: u32,
    error: DNSServiceErrorType,
    fullname: *const c_char,
    host_target: *const c_char,
    port: u16,
    _txt_len: u16,
    _txt_record: *const c_uchar,
    context: *mut c_void,
) {
    println!("resolve_callback()");

    let ctx = BonjourBrowserContext::from_raw(context);

    println!("context = {:?}", ctx);

    if error != 0 {
        panic!("error reported by resolve_callback: (code: {})", error);
    }

    let fullname_r = CStr::from_ptr(fullname).to_str().unwrap();
    let host_target_r = CStr::from_ptr(host_target).to_str().unwrap();

    println!("fullname = {:?}", fullname_r);
    println!("host_target = {:?}\n", host_target_r);

    ctx.resolved_port = port;

    ManagedDNSServiceRef::default()
        .get_address_info(
            GetAddressInfoParams::builder()
                .flags(bonjour_sys::kDNSServiceFlagsForceMulticast)
                .interface_index(interface_index)
                .protocol(0)
                .hostname(host_target)
                .callback(Some(get_address_info_callback))
                // .context(Box::into_raw(ctx) as *mut c_void)
                .context(context)
                .build()
                .expect("could not build GetAddressInfoParams"),
        )
        .unwrap();

    // free context
    Box::from_raw(context as *mut BonjourBrowserContext);
}

extern "C" {
    fn inet_ntoa(addr: *const libc::in_addr) -> *const c_char;
}

unsafe extern "C" fn get_address_info_callback(
    _sd_ref: DNSServiceRef,
    _flags: DNSServiceFlags,
    _interface_index: u32,
    error: DNSServiceErrorType,
    hostname: *const c_char,
    address: *const sockaddr,
    _ttl: u32,
    context: *mut c_void,
) {
    println!("get_address_info_callback()");

    let ctx = BonjourBrowserContext::from_raw(context);

    // this callback runs multiple times for some reason
    if ctx.resolved_name.is_none() {
        println!("duplicate call");
        return;
    }

    println!("context = {:?}", ctx);

    if error != 0 {
        panic!(
            "get_address_info_callback() reported error (code: {})",
            error
        );
    }

    let address_c = address as *const sockaddr_in;
    let address_c_str = inet_ntoa(&(*address_c).sin_addr as *const in_addr);
    let address_r = CStr::from_ptr(address_c_str).to_str().unwrap();

    let hostname_r = CStr::from_ptr(hostname).to_str().unwrap();
    let hostname_string = String::from(CStr::from_ptr(hostname).to_str().unwrap());

    println!("address = {:?}", address_r);
    println!("hostname = {:?}", hostname_r);

    let domain = util::normalize_domain(&ctx.resolved_domain.take().unwrap());

    println!("domain = {:?}", domain);

    let result = ServiceResolution::builder()
        .name(ctx.resolved_name.take().unwrap())
        .kind(ctx.resolved_kind.take().unwrap())
        .domain(domain)
        .host_name(hostname_string)
        .address(String::from(address_r))
        .port(ctx.resolved_port)
        .build()
        .expect("could not build ServiceResolution");

    if let Some(f) = &ctx.resolver_found_callback {
        f(result);
    }

    println!();
}