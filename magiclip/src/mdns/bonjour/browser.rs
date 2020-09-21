use super::compat;
use super::service_ref::{
    BrowseServicesParams, GetAddressInfoParams, ManagedDNSServiceRef, ServiceResolveParams,
};
use crate::builder::BuilderDelegate;
use crate::ffi::{cstr, FromRaw};
use crate::mdns::{ResolverFoundCallback, ServiceResolution};
use bonjour_sys::{sockaddr, DNSServiceErrorType, DNSServiceFlags, DNSServiceRef};
use libc::{c_char, c_uchar, c_void, in_addr, sockaddr_in};
use std::ffi::CString;
use std::fmt::{self, Formatter};
use std::ptr;

#[derive(Debug)]
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
        debug!("Browsing services: {:?}", self);

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
    let ctx = BonjourBrowserContext::from_raw(context);

    if error != 0 {
        panic!("browse_callback() reported error (code: {})", error);
    }

    ctx.resolved_name = Some(cstr::copy_raw(name));
    ctx.resolved_kind = Some(cstr::copy_raw(regtype));
    ctx.resolved_domain = Some(cstr::copy_raw(domain));

    ManagedDNSServiceRef::default()
        .resolve_service(
            ServiceResolveParams::builder()
                .flags(bonjour_sys::kDNSServiceFlagsForceMulticast)
                .interface_index(interface_index)
                .name(name)
                .regtype(regtype)
                .domain(domain)
                .callback(Some(resolve_callback))
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
    _fullname: *const c_char,
    host_target: *const c_char,
    port: u16,
    _txt_len: u16,
    _txt_record: *const c_uchar,
    context: *mut c_void,
) {
    let ctx = BonjourBrowserContext::from_raw(context);

    if error != 0 {
        panic!("error reported by resolve_callback: (code: {})", error);
    }

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
    let ctx = BonjourBrowserContext::from_raw(context);

    // this callback runs multiple times for some reason
    if ctx.resolved_name.is_none() {
        return;
    }

    if error != 0 {
        panic!(
            "get_address_info_callback() reported error (code: {})",
            error
        );
    }

    let ip = get_ip(address as *const sockaddr_in);
    let hostname = cstr::copy_raw(hostname);
    let domain = compat::normalize_domain(&ctx.resolved_domain.take().unwrap());

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
}

extern "C" {
    fn inet_ntoa(addr: *const libc::in_addr) -> *const c_char;
}

unsafe fn get_ip(address: *const sockaddr_in) -> String {
    let raw = inet_ntoa(&(*address).sin_addr as *const in_addr);
    String::from(cstr::raw_to_str(raw))
}
