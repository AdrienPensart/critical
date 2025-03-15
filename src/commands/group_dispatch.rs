use crate::music::errors::CriticalErrorKind;
use async_trait::async_trait;
use enum_dispatch::enum_dispatch;

use super::opts::Config;

#[async_trait]
#[enum_dispatch]
pub trait GroupDispatch {
    async fn dispatch(self, config: Config) -> Result<(), CriticalErrorKind>;
}
