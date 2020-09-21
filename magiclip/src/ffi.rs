use libc::c_void;

pub trait FromRaw<T> {
    unsafe fn from_raw<'a>(raw: *mut c_void) -> &'a mut T {
        &mut *(raw as *mut T)
    }
}

pub trait CloneRaw<T: FromRaw<T> + Clone> {
    unsafe fn clone_raw<'a>(raw: *mut c_void) -> Box<T> {
        Box::new(T::from_raw(raw).clone())
    }
}

pub mod cstr {
    use libc::c_char;
    use std::ffi::CStr;

    pub unsafe fn raw_to_str<'a>(s: *const c_char) -> &'a str {
        CStr::from_ptr(s).to_str().unwrap()
    }

    pub unsafe fn copy_raw(s: *const c_char) -> String {
        String::from(raw_to_str(s))
    }
}
