use crate::music::{config::Config, errors::CriticalErrorKind};
use async_trait::async_trait;
use enum_dispatch::enum_dispatch;

#[async_trait]
#[enum_dispatch]
pub trait GroupDispatch {
    async fn dispatch(self, config: Config) -> Result<(), CriticalErrorKind>;
}
