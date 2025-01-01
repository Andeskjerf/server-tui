use std::{collections::HashMap, sync::Arc, time::Duration};

use sysinfo::{ProcessRefreshKind, RefreshKind, System};
use tokio::{sync::Mutex, time::sleep};

pub struct ProcessWatcher {
    system: Arc<Mutex<System>>,
    pub status: Arc<Mutex<HashMap<String, i64>>>,
}

impl ProcessWatcher {
    pub fn new() -> Self {
        Self {
            system: Arc::new(Mutex::new(System::new_with_specifics(
                RefreshKind::nothing().with_processes(ProcessRefreshKind::everything()),
            ))),
            status: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn cleanup_task(&self) {
        const CLEANUP_INTERVAL: u8 = 2;
        let status = Arc::clone(&self.status);
        tokio::spawn(async move {
            loop {
                let mut lock = status.lock().await;
                let now = chrono::Utc::now().timestamp();
                (*lock).clone().into_iter().for_each(|(key, value)| {
                    if now - value >= CLEANUP_INTERVAL as i64 {
                        (*lock).remove(&key);
                    }
                });
                sleep(Duration::from_millis(100)).await;
            }
        });
    }

    pub fn watch_process(&self, process_name: String) {
        let status = Arc::clone(&self.status);
        let system = Arc::clone(&self.system);
        tokio::spawn(async move {
            loop {
                let mut lock = system.lock().await;
                (*lock).refresh_processes(sysinfo::ProcessesToUpdate::All, true);
                for process in (*lock).processes().values() {
                    let exe = match process.exe() {
                        // FIXME: should probably deal with this properly?
                        Some(res) => res.file_name().unwrap().to_str().unwrap(),
                        None => continue,
                    };

                    if exe.contains(&process_name) {
                        status
                            .lock()
                            .await
                            .insert(process_name.clone(), chrono::Utc::now().timestamp());
                        break;
                    }
                }
                sleep(Duration::from_millis(100)).await;
            }
        });
    }
}
