use std::{
    collections::HashMap, sync::{Arc, Mutex}, thread::sleep, time::Duration
};

use crate::traits::runnable::Runnable;
use sysinfo::{ProcessRefreshKind, RefreshKind, System};
// use tokio::time::sleep;

use super::event_bus::EventBus;

pub struct ProcessWatcher {
    system: Arc<Mutex<System>>,
    to_watch: Arc<Mutex<Vec<String>>>,
    event_bus: Arc<Mutex<EventBus>>,
    pub status: Arc<Mutex<HashMap<String, i64>>>,
}

impl ProcessWatcher {
    pub fn new(event_bus: Arc<Mutex<EventBus>>, processes_to_watch: Vec<String>) -> Self {
        Self {
            system: Arc::new(Mutex::new(System::new_with_specifics(
                RefreshKind::nothing().with_processes(ProcessRefreshKind::everything()),
            ))),
            event_bus,
            to_watch: Arc::new(Mutex::new(processes_to_watch)),
            status: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn cleanup_task(&self) {
        const CLEANUP_INTERVAL: u8 = 2;
        let status = Arc::clone(&self.status);
        tokio::spawn(async move {
            loop {
                let mut lock = status.lock().unwrap();
                let now = chrono::Utc::now().timestamp();
                (*lock).clone().into_iter().for_each(|(key, value)| {
                    if now - value >= CLEANUP_INTERVAL as i64 {
                        (*lock).remove(&key);
                    }
                });
                sleep(Duration::from_millis(100));
            }
        });
    }
}

impl Runnable for ProcessWatcher {
    fn run(&self) {
        // let status = Arc::clone(&self.status);
        let event_bus = Arc::clone(&self.event_bus);
        let system = Arc::clone(&self.system);
        let to_watch = Arc::clone(&self.to_watch);

        self.cleanup_task();

        tokio::spawn(async move {
            loop {
                let mut lock = system.lock().unwrap();
                (*lock).refresh_processes(sysinfo::ProcessesToUpdate::All, true);
                for process in (*lock).processes().values() {
                    let exe = match process.exe() {
                        // FIXME: should probably deal with this properly?
                        Some(res) => res.file_name().unwrap().to_str().unwrap(),
                        None => continue,
                    };

                    let name = process.name().to_str().unwrap();
                    let mut found = false;

                    for elem in to_watch.lock().unwrap().iter() {
                        if name.to_lowercase().contains(&elem.to_lowercase()) {
                            // status
                            //     .lock()
                            //     .await
                            //     .insert(elem.clone(), chrono::Utc::now().timestamp());
                            event_bus
                                .lock()
                                .unwrap()
                                .publish("process_watcher", "".as_bytes().to_vec());
                            found = true;
                            println!("{elem}, {name}, {exe}");
                            break;
                        }
                    }

                    if found {
                        break;
                    }
                }
                sleep(Duration::from_millis(100));
            }
        });
    }
}
