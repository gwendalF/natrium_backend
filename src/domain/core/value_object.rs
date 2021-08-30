pub trait ValueObject<T, E> {
    fn value(&self) -> Result<&T, &E>;
    fn is_valid(&self) -> bool;
}
