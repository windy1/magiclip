use avahi_sys::avahi_strerror;
use std::ffi::CStr;

pub fn get_error<'a>(code: i32) -> &'a str {
    unsafe {
        CStr::from_ptr(avahi_strerror(code))
            .to_str()
            .expect("could not fetch Avahi error string")
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