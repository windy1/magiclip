use bonjour_sys::{
    DNSServiceBrowse, DNSServiceBrowseReply, DNSServiceFlags, DNSServiceProcessResult,
    DNSServiceRef, DNSServiceRefDeallocate, DNSServiceRegister, DNSServiceRegisterReply,
    DNSServiceResolve, DNSServiceResolveReply,
};
use libc::{c_char, c_void};
use std::ptr;

pub struct ManagedDNSServiceRef {
    service: DNSServiceRef,
}

#[derive(Builder)]
pub struct RegisterServiceParams {
    flags: DNSServiceFlags,
    interface_index: u32,
    name: *const c_char,
    regtype: *const c_char,
    domain: *const c_char,
    host: *const c_char,
    port: u16,
    txt_len: u16,
    txt_record: *const c_void,
    callback: DNSServiceRegisterReply,
    context: *mut c_void,
}

#[derive(Builder)]
pub struct BrowseServicesParams {
    flags: DNSServiceFlags,
    interface_index: u32,
    regtype: *const c_char,
    domain: *const c_char,
    callback: DNSServiceBrowseReply,
    context: *mut c_void,
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

impl ManagedDNSServiceRef {
    pub fn new() -> Self {
        Self {
            service: ptr::null_mut(),
        }
    }

    pub fn register_service(
        &mut self,
        RegisterServiceParams {
            flags,
            interface_index,
            name,
            regtype,
            domain,
            host,
            port,
            txt_len,
            txt_record,
            callback,
            context,
        }: RegisterServiceParams,
    ) -> Result<(), String> {
        let err = unsafe {
            DNSServiceRegister(
                &mut self.service as *mut DNSServiceRef,
                flags,
                interface_index,
                name,
                regtype,
                domain,
                host,
                port,
                txt_len,
                txt_record,
                callback,
                context,
            )
        };

        if err != 0 {
            return Err(format!("could not register service (code: {})", err).to_string());
        }

        loop {
            self.process_result()?
        }
    }

    pub fn browse_services(
        &mut self,
        BrowseServicesParams {
            flags,
            interface_index,
            regtype,
            domain,
            callback,
            context,
        }: BrowseServicesParams,
    ) -> Result<(), String> {
        let err = unsafe {
            DNSServiceBrowse(
                &mut self.service as *mut DNSServiceRef,
                flags,
                interface_index,
                regtype,
                domain,
                callback,
                context,
            )
        };

        if err != 0 {
            return Err(format!("could not browse services (code: {})", err).to_string());
        }

        loop {
            self.process_result()?
        }
    }

    pub fn resolve_service(
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

        self.process_result()
    }

    fn process_result(&self) -> Result<(), String> {
        let err = unsafe { DNSServiceProcessResult(self.service) };
        if err != 0 {
            Err(format!("could not process service result (code: {})", err))
        } else {
            Ok(())
        }
    }
}

impl Drop for ManagedDNSServiceRef {
    fn drop(&mut self) {
        unsafe {
            if self.service != ptr::null_mut() {
                DNSServiceRefDeallocate(self.service);
            }
        }
    }
}

impl RegisterServiceParams {
    pub fn builder() -> RegisterServiceParamsBuilder {
        RegisterServiceParamsBuilder::default()
    }
}

impl BrowseServicesParams {
    pub fn builder() -> BrowseServicesParamsBuilder {
        BrowseServicesParamsBuilder::default()
    }
}

impl ServiceResolveParams {
    pub fn builder() -> ServiceResolveParamsBuilder {
        ServiceResolveParamsBuilder::default()
    }
}
