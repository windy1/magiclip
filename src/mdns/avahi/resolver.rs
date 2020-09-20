use super::client::ManagedAvahiClient;
use crate::util::BuilderDelegate;
use avahi_sys::{
    avahi_service_resolver_free, avahi_service_resolver_new, AvahiIfIndex, AvahiLookupFlags,
    AvahiProtocol, AvahiServiceResolver, AvahiServiceResolverCallback,
};
use libc::{c_char, c_void};
use std::ptr;

pub struct ManagedAvahiServiceResolver {
    resolver: *mut AvahiServiceResolver,
}

impl ManagedAvahiServiceResolver {
    pub fn new(
        ManagedAvahiServiceResolverParams {
            client,
            interface,
            protocol,
            name,
            kind,
            domain,
            aprotocol,
            flags,
            callback,
            userdata,
        }: ManagedAvahiServiceResolverParams,
    ) -> Result<Self, String> {
        let resolver = unsafe {
            avahi_service_resolver_new(
                client.client,
                interface,
                protocol,
                name,
                kind,
                domain,
                aprotocol,
                flags,
                callback,
                userdata,
            )
        };

        if resolver == ptr::null_mut() {
            Err("could not initialize AvahiServiceResolver".to_string())
        } else {
            Ok(Self { resolver })
        }
    }
}

impl Drop for ManagedAvahiServiceResolver {
    fn drop(&mut self) {
        unsafe { avahi_service_resolver_free(self.resolver) };
    }
}

#[derive(Builder)]
pub struct ManagedAvahiServiceResolverParams<'a> {
    client: &'a ManagedAvahiClient,
    interface: AvahiIfIndex,
    protocol: AvahiProtocol,
    name: *const c_char,
    kind: *const c_char,
    domain: *const c_char,
    aprotocol: AvahiProtocol,
    flags: AvahiLookupFlags,
    callback: AvahiServiceResolverCallback,
    userdata: *mut c_void,
}

impl<'a> BuilderDelegate<ManagedAvahiServiceResolverParamsBuilder<'a>>
    for ManagedAvahiServiceResolverParams<'a>
{
}
