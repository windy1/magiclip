use super::constants;
use avahi_sys::{avahi_address_snprint, AvahiAddress};
use libc::c_char;
use std::ffi::CString;
use std::mem;

pub unsafe fn get_ip(addr: *const AvahiAddress) -> String {
    let addr_str = CString::from_vec_unchecked(vec![0; constants::AVAHI_ADDRESS_STR_MAX]);

    avahi_address_snprint(
        addr_str.as_ptr() as *mut c_char,
        mem::size_of_val(&addr_str),
        addr,
    );

    String::from(addr_str.to_str().unwrap())
        .trim_matches(char::from(0))
        .to_string()
}
