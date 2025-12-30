use std::sync::Arc;
use futures::future::BoxFuture;
use futures::FutureExt;
use crate::state::AppState;
use crate::task::ScheduledTask;

pub struct CleanupTask;
impl ScheduledTask for CleanupTask {
    fn run(&self, state: Arc<AppState>) -> BoxFuture<'static, ()> {
        async move {
            match state.db.sessions.remove_expired().await {
                Ok(count) => {
                    log::info!("CleanupTask: Removed {} expired sessions.", count);
                }
                Err(e) => {
                    log::error!("CleanupTask: Failed to remove expired sessions: {}", e);
                }
            }

            match state.db.sessions.remove_expired().await {
                Ok(count) => {
                    log::info!("CleanupTask: Removed {} expired codes.", count);
                }
                Err(e) => {
                    log::error!("CleanupTask: Failed to remove expired codes: {}", e);
                }
            }
            
            
            // cleanup unverified accounts
            match state.db.pre_user_repo.remove_expired().await {
                Ok(count) => {
                    log::info!("CleanupTask: Deleted {} pre-reg users that were not verified after 24 hours.", count);
                }
                Err(e) => {
                    log::error!("CleanupTask: Failed to remove pre-reg users that were not verified: {}", e);
                }
            }
        }
            .boxed()
    }

    fn name(&self) -> &str {
        "CleanupTask"
    }

    fn interval_seconds(&self) -> u64 {
        3600
    }
}