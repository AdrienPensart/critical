pub trait ErrOnSome {
    fn err_on_some<F, E>(&self, f: F) -> Result<(), E>
    where
        F: FnOnce() -> Result<(), E>;
}

impl<T> ErrOnSome for Option<T> {
    fn err_on_some<F, E>(&self, f: F) -> Result<(), E>
    where
        F: FnOnce() -> Result<(), E>,
    {
        match self {
            None => Ok(()),
            Some(_) => f(),
        }
    }
}
