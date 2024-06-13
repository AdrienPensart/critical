use async_trait::async_trait;
use enum_dispatch::enum_dispatch;

use crate::errors::CriticalErrorKind;

#[async_trait]
#[enum_dispatch]
pub trait GroupDispatch {
    async fn dispatch(self, client: edgedb_tokio::Client) -> Result<(), CriticalErrorKind>;
}
