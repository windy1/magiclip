use crate::mdns::err::{ErrorCallback, HandleError};
use bonjour_sys::{
    kDNSServiceProperty_DaemonVersion, DNSServiceCreateConnection, DNSServiceErrorType,
    DNSServiceFlags, DNSServiceGetProperty, DNSServiceProcessResult, DNSServiceRef,
    DNSServiceRefDeallocate, DNSServiceRegister, DNSServiceRegisterReply,
};
use libc::{c_char, c_void};
use std::ffi::CString;
use std::{mem, ptr};

const BOUNJOUR_IF_UNSPEC: u32 = 0;
const BONJOUR_RENAME_FLAGS: DNSServiceFlags = 0;

pub struct MdnsService {
    service: DNSServiceRef,
    kind: CString,
    port: u16,
    context: *mut BonjourServiceContext,
}

struct BonjourServiceContext {
    error_callback: Option<Box<ErrorCallback>>,
}

impl MdnsService {
    pub fn new(kind: &str, port: u16) -> Self {
        Self {
            service: ptr::null_mut(),
            kind: CString::new(kind).unwrap(),
            port,
            context: Box::into_raw(Box::new(BonjourServiceContext {
                error_callback: None,
            })),
        }
    }

    pub fn set_error_callback(&mut self, error_callback: Box<ErrorCallback>) {
        unsafe { (*self.context).error_callback = Some(error_callback) };
    }

    pub fn start(&mut self) -> Result<(), String> {
        println!("registering service");

        let err = unsafe {
            DNSServiceRegister(
                &mut self.service as *mut DNSServiceRef, // sdRef
                BONJOUR_RENAME_FLAGS,                    // flags
                BOUNJOUR_IF_UNSPEC,                      // interfaceIndex
                ptr::null(),                             // name
                self.kind.as_ptr(),                      // regtype
                ptr::null(),                             // domain
                ptr::null(),                             // host
                self.port,                               // port
                0,                                       // txtLen
                ptr::null(),                             // txtRecord
                Some(register_callback),                 // callback
                ptr::null_mut(),                         // context
            )
        };

        if (err != 0) {
            return Err(
                format!("could not register service with error code: `{0}`", err).to_string(),
            );
        }

        loop {
            let err = unsafe { DNSServiceProcessResult(self.service) };

            if err != 0 {
                return Err(format!("could not process service result: `{0}`", err));
            }
        }
    }
}

impl Drop for MdnsService {
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

impl HandleError for BonjourServiceContext {
    fn error_callback(&self) -> Option<&Box<ErrorCallback>> {
        self.error_callback.as_ref()
    }
}

extern "C" fn register_callback(
    _sd_ref: DNSServiceRef,
    _flags: DNSServiceFlags,
    error: DNSServiceErrorType,
    _name: *const c_char,
    _regtype: *const c_char,
    _domain: *const c_char,
    context: *mut c_void,
) {
    let context = unsafe { &mut *(context as *mut BonjourServiceContext) };
    println!("register_callback");
    if error != 0 {
        context.handle_error(&format!(
            "register_callback reported error (code: {0})",
            error
        ));
    }
}
