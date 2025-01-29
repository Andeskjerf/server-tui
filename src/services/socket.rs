use std::{
    collections::HashMap,
    env::temp_dir,
    path::Path,
    sync::{Arc, Mutex},
};

use tokio::sync::Mutex as TokioMutex;
use tokio::{fs, io::AsyncReadExt, net::UnixListener};

use crate::models::socket_message::SocketMessage;
use crate::traits::runnable::Runnable;

use super::event_bus::EventBus;

pub struct SocketService {
    pub queue: Arc<Mutex<HashMap<String, String>>>,
    listener: Arc<TokioMutex<UnixListener>>,
    event_bus: Arc<Mutex<EventBus>>,
    socket_name: String,
}

impl SocketService {
    pub async fn new(event_bus: Arc<Mutex<EventBus>>, socket_name: &str) -> Self {
        Self {
            queue: Arc::new(Mutex::new(HashMap::new())),
            listener: Arc::new(TokioMutex::new(SocketService::init_socket(socket_name).await)),
            event_bus,
            socket_name: socket_name.to_string(),
        }
    }

    async fn init_socket(socket_name: &str) -> UnixListener {
        let dir = temp_dir();
        let bind_path = dir.join(socket_name);
        if Path::new(&bind_path).exists() {
            fs::remove_file(&bind_path)
                .await
                .expect("unable to remove old socket on init");
        }

        UnixListener::bind(&bind_path).expect("unable to initialize socket listener")
    }

    fn process_message(buffer: Vec<u8>, queue: Arc<Mutex<HashMap<String, String>>>) {
        if let Ok(string) = String::from_utf8(buffer) {
            let msg: SocketMessage = serde_json::from_str(
                // remove any additional zeros from the buffer
                string.trim_end_matches(char::from(0)),
            )
            .unwrap();

            let mut lock = queue.lock().unwrap();

            if msg.status.to_lowercase() != "done" {
                (*lock).insert(msg.title, msg.status);
                return;
            }

            (*lock).remove(&msg.title);
        }
    }
}

impl Runnable for SocketService {
    // TODO: needs to handle errors / bad input / kick out clients
    // FIXME: very nested
    fn run(&self) {
        let listener = Arc::clone(&self.listener);
        let queue = Arc::clone(&self.queue);
        tokio::spawn(async move {
            while let Ok((mut stream, _)) = listener.lock().await.accept().await {
                let mut buffer = vec![0u8; 1024];
                {
                    let queue = Arc::clone(&queue);
                    tokio::spawn(async move {
                        match stream.read(&mut buffer).await {
                            Err(err) => {
                                println!("err: {err}");
                            }
                            Ok(_) => {
                                // static function since dealing with borrowed self is hard
                                SocketService::process_message(buffer, queue);
                            }
                        }
                    });
                }
            }
        });
    }
}
