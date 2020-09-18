use avahi_sys::{
    avahi_client_new, avahi_service_browser_new, avahi_simple_poll_get, avahi_simple_poll_new,
    avahi_strerror, AvahiClient, AvahiClientCallback, AvahiClientFlags, AvahiIfIndex,
    AvahiLookupFlags, AvahiProtocol, AvahiServiceBrowser, AvahiServiceBrowserCallback,
    AvahiSimplePoll,
};
use libc::{c_char, c_int, c_void};
use std::convert::TryInto;
use std::ffi::CStr;
use std::ptr;

pub const AVAHI_IF_UNSPEC: i32 = -1;
pub const AVAHI_PROTO_UNSPEC: i32 = -1;
pub const AVAHI_ERR_COLLISION: i32 = -8;
pub const AVAHI_ADDRESS_STR_MAX: usize = 40;

pub fn new_poller() -> Result<*mut AvahiSimplePoll, String> {
    let poller = unsafe { avahi_simple_poll_new() };
    if poller == ptr::null_mut() {
        Err("could not initialize Avahi simple poll".to_string())
    } else {
        Ok(poller)
    }
}

pub fn get_error<'a>(code: i32) -> &'a str {
    unsafe {
        CStr::from_ptr(avahi_strerror(code))
            .to_str()
            .expect("could not fetch Avahi error string")
    }
}

#[derive(Builder)]
pub struct AvahiClientParams {
    poller: *mut AvahiSimplePoll,
    callback: AvahiClientCallback,
    context: *mut c_void,
}

impl AvahiClientParams {
    pub fn builder() -> AvahiClientParamsBuilder {
        AvahiClientParamsBuilder::default()
    }
}

impl TryInto<*mut AvahiClient> for AvahiClientParams {
    type Error = String;

    fn try_into(self) -> Result<*mut AvahiClient, String> {
        let mut err: c_int = 0;

        let client = unsafe {
            avahi_client_new(
                avahi_simple_poll_get(self.poller), // poll_api
                AvahiClientFlags(0),                // flags
                self.callback,                      // callback
                self.context,                       // userdata
                &mut err,                           // error
            )
        };

        if client == ptr::null_mut() {
            return Err("could not initialize AvahiClient".to_string());
        }

        match err {
            0 => Ok(client),
            _ => Err(format!("could not initialize AvahiClient (error: {})", err)),
        }
    }
}

#[derive(Builder)]
pub struct AvahiServiceBrowserParams {
    client: *mut AvahiClient,
    interface: AvahiIfIndex,
    protocol: AvahiProtocol,
    kind: *const c_char,
    domain: *const c_char,
    flags: AvahiLookupFlags,
    callback: AvahiServiceBrowserCallback,
    context: *mut c_void,
}

impl AvahiServiceBrowserParams {
    pub fn builder() -> AvahiServiceBrowserParamsBuilder {
        AvahiServiceBrowserParamsBuilder::default()
    }
}

impl TryInto<*mut AvahiServiceBrowser> for AvahiServiceBrowserParams {
    type Error = String;

    fn try_into(self) -> Result<*mut AvahiServiceBrowser, String> {
        let browser = unsafe {
            avahi_service_browser_new(
                self.client,
                self.interface,
                self.protocol,
                self.kind,
                self.domain,
                self.flags,
                self.callback,
                self.context,
            )
        };

        if browser == ptr::null_mut() {
            Err("could not initialize Avahi service browser".to_string())
        } else {
            Ok(browser)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_error_returns_valid_error_string() {
        assert_eq!(get_error(avahi_sys::AVAHI_ERR_FAILURE), "Operation failed");
    }
}
