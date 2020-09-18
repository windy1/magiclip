pub type ErrorCallback = dyn Fn(&str);

pub trait HandleError {
    fn error_callback(&self) -> Option<&Box<ErrorCallback>>;

    fn handle_error(&self, err: &str) {
        match self.error_callback() {
            Some(f) => f(err),
            None => panic!("unhandled error: `{}`", err),
        };
    }
}
