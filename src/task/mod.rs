pub mod cleanup;

use std::sync::Arc;
use async_trait::async_trait;
use futures::future::BoxFuture;
use crate::state::AppState;

#[async_trait]
pub trait ScheduledTask: Send + Sync + 'static {
    fn run(&self, state: Arc<AppState>) -> BoxFuture<'static, ()>;
    fn name(&self) -> &str;
    fn interval_seconds(&self) -> u64;
}
