use anyhow::Result;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait GroupDispatch{
    fn dispatch(self) -> Result<()>;
}
