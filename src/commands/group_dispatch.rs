use crate::music::errors::CriticalErrorKind;
use async_trait::async_trait;
use enum_dispatch::enum_dispatch;

#[async_trait]
#[enum_dispatch]
pub trait GroupDispatch {
    async fn dispatch(self, client: gel_tokio::Client, dry: bool) -> Result<(), CriticalErrorKind>;
}
