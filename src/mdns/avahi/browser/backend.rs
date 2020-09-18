use avahi_sys::{
    avahi_service_browser_new, AvahiClient, AvahiIfIndex, AvahiLookupFlags, AvahiProtocol,
    AvahiServiceBrowser, AvahiServiceBrowserCallback,
};
use libc::{c_char, c_void};
use std::convert::TryInto;
use std::ptr;

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
