use libc::c_void;

pub trait BuilderDelegate<T: Default> {
    fn builder() -> T {
        T::default()
    }
}

pub trait FromRaw<T> {
    fn from_raw<'a>(raw: *mut c_void) -> &'a mut T {
        unsafe { &mut *(raw as *mut T) }
    }
}
