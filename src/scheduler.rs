use std::sync::Arc;
use std::time::Duration;
use futures::FutureExt;
use tokio::task::JoinHandle;
use tokio::time::interval;
use crate::state::AppState;
use crate::task::cleanup::CleanupTask;
use crate::task::ScheduledTask;

pub struct Scheduler {
    state: Arc<AppState>,
    handles: Vec<JoinHandle<()>>,
}

impl Scheduler {
    pub fn new(state: Arc<AppState>) -> Self {
        Scheduler {
            state,
            handles: Vec::new(),
        }
    }
    
    pub fn start_all(&mut self) {
        self.spawn_task(CleanupTask)
    }


    fn spawn_task(&mut self, task: impl ScheduledTask + 'static) {
        let task: Arc<dyn ScheduledTask> = Arc::new(task);
        let state = self.state.clone();
        let name = task.name().to_string();
        let interval_duration = Duration::from_secs(task.interval_seconds());
        let task_clone = task.clone(); 

        let handle = tokio::spawn(async move {
            let mut ticker = interval(interval_duration);
            loop {
                ticker.tick().await;
                log::info!("Running scheduled task '{}'", name);

                let task = task_clone.clone();
                if let Err(e) = std::panic::AssertUnwindSafe(task.run(state.clone()))
                    .catch_unwind()
                    .await
                {
                    log::error!("Task '{}' panicked: {:?}", name, e);
                }
            }
        });

        self.handles.push(handle);
    }
}
