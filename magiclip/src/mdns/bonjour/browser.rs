use super::backend::{
    BrowseServicesParams, GetAddressInfoParams, ManagedDNSServiceRef, ServiceResolveParams,
};
use super::util;
use crate::builder::BuilderDelegate;
use crate::ffi::cstr;
use crate::mdns::{ResolverFoundCallback, ServiceResolution};
use bonjour_sys::{sockaddr, DNSServiceErrorType, DNSServiceFlags, DNSServiceRef};
use libc::{c_char, c_uchar, c_void, in_addr, sockaddr_in};
use std::ffi::CString;
use std::fmt::{self, Formatter};
use std::ptr;

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
        unsafe { (*self.context).resolver_found_callback = Some(resolver_found_callback) };
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

#[derive(Default, FromRaw)]
struct BonjourBrowserContext {
    resolver_found_callback: Option<Box<ResolverFoundCallback>>,
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

    let ctx = BonjourBrowserContext::from_raw(context);

    if error != 0 {
        panic!("browse_callback() reported error (code: {})", error);
    }

    ctx.resolved_name = Some(cstr::copy_raw(name));
    ctx.resolved_kind = Some(cstr::copy_raw(regtype));
    ctx.resolved_domain = Some(cstr::copy_raw(domain));

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
                // .context(Box::into_raw(ctx) as *mut c_void)
                .context(context)
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

    let fullname_r = cstr::raw_to_str(fullname);
    let host_target_r = cstr::raw_to_str(host_target);

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
                .context(context)
                .build()
                .expect("could not build GetAddressInfoParams"),
        )
        .unwrap();
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

    let ip = get_ip(address as *const sockaddr_in);
    let hostname = cstr::copy_raw(hostname);
    let domain = util::normalize_domain(&ctx.resolved_domain.take().unwrap());

    println!("address = {:?}", ip);
    println!("hostname = {:?}", hostname);
    println!("domain = {:?}", domain);

    let result = ServiceResolution::builder()
        .name(ctx.resolved_name.take().unwrap())
        .kind(ctx.resolved_kind.take().unwrap())
        .domain(domain)
        .host_name(hostname)
        .address(ip)
        .port(ctx.resolved_port)
        .build()
        .expect("could not build ServiceResolution");

    if let Some(f) = &ctx.resolver_found_callback {
        f(result);
    }

    println!();
}

extern "C" {
    fn inet_ntoa(addr: *const libc::in_addr) -> *const c_char;
}

unsafe fn get_ip(address: *const sockaddr_in) -> String {
    let raw = inet_ntoa(&(*address).sin_addr as *const in_addr);
    String::from(cstr::raw_to_str(raw))
}
