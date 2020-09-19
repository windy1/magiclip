use bonjour_sys::{
    DNSServiceFlags, DNSServiceProcessResult, DNSServiceRef, DNSServiceRefDeallocate,
    DNSServiceResolve, DNSServiceResolveReply,
};
use libc::{c_char, c_void};
use std::ptr;

pub struct MdnsServiceResolver {
    service: DNSServiceRef,
}

#[derive(Builder)]
pub struct ServiceResolveParams {
    flags: DNSServiceFlags,
    interface_index: u32,
    name: *const c_char,
    regtype: *const c_char,
    domain: *const c_char,
    callback: DNSServiceResolveReply,
    context: *mut c_void,
}

impl MdnsServiceResolver {
    pub fn new() -> Self {
        Self {
            service: ptr::null_mut(),
        }
    }

    pub fn resolve(
        &mut self,
        ServiceResolveParams {
            flags,
            interface_index,
            name,
            regtype,
            domain,
            callback,
            context,
        }: ServiceResolveParams,
    ) -> Result<(), String> {
        let error = unsafe {
            DNSServiceResolve(
                &mut self.service as *mut DNSServiceRef,
                flags,
                interface_index,
                name,
                regtype,
                domain,
                callback,
                context,
            )
        };

        if error != 0 {
            return Err(format!(
                "DNSServiceResolve reported error (code: {})",
                error
            ));
        }

        let err = unsafe { DNSServiceProcessResult(self.service) };

        if err != 0 {
            Err(format!("could not start processing loop: `{}`", err).to_string())
        } else {
            Ok(())
        }
    }
}

impl Drop for MdnsServiceResolver {
    fn drop(&mut self) {
        unsafe {
            if self.service != ptr::null_mut() {
                DNSServiceRefDeallocate(self.service);
            }
        }
    }
}

impl ServiceResolveParams {
    pub fn builder() -> ServiceResolveParamsBuilder {
        ServiceResolveParamsBuilder::default()
    }
}
