use std::{collections::HashMap, sync::Arc, time::Duration};

use sysinfo::{ProcessRefreshKind, RefreshKind, System};
use tokio::{sync::Mutex, time::sleep};

pub struct ProcessWatcher {
    system: Arc<Mutex<System>>,
    to_watch: Arc<Mutex<Vec<String>>>,
    pub status: Arc<Mutex<HashMap<String, i64>>>,
}

impl ProcessWatcher {
    pub fn new() -> Self {
        Self {
            system: Arc::new(Mutex::new(System::new_with_specifics(
                RefreshKind::nothing().with_processes(ProcessRefreshKind::everything()),
            ))),
            to_watch: Arc::new(Mutex::new(vec![])),
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

    pub async fn watch_process(&self, name: &str) {
        self.to_watch.lock().await.push(name.to_owned());
    }

    pub fn run(&self) {
        let status = Arc::clone(&self.status);
        let system = Arc::clone(&self.system);
        let to_watch = Arc::clone(&self.to_watch);
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

                    let name = process.name().to_str().unwrap();
                    let mut found = false;

                    for elem in to_watch.lock().await.iter() {
                        if name.to_lowercase().contains(&elem.to_lowercase()) {
                            status
                                .lock()
                                .await
                                .insert(elem.clone(), chrono::Utc::now().timestamp());
                            found = true;
                            println!("{elem}, {name}, {exe}");
                            break;
                        }
                    }

                    if found {
                        break;
                    }
                }
                sleep(Duration::from_millis(100)).await;
            }
        });
    }
}
