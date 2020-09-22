pub trait BuilderDelegate<T: Default> {
    fn builder() -> T {
        T::default()
    }
}
