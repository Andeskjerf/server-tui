use std::{
    env::temp_dir,
    path::Path,
    sync::{Arc, Mutex},
};

use tokio::sync::Mutex as TokioMutex;
use tokio::{fs, io::AsyncReadExt, net::UnixListener};

use crate::models::{
    event_bus_field_type::EventFieldType, event_bus_message::EventBusMessage,
    event_type::EventType, socket_message::SocketMessage,
};
use crate::traits::runnable::Runnable;

use super::event_bus::EventBus;

pub const EVENT_TOPIC: &str = "socket_service";
pub const SOCKET_DONE_TEXT: &str = "done";

pub struct SocketService {
    listener: Arc<TokioMutex<UnixListener>>,
    event_bus: Arc<Mutex<EventBus>>,
}

impl SocketService {
    pub async fn new(event_bus: Arc<Mutex<EventBus>>, socket_name: &str) -> Self {
        Self {
            listener: Arc::new(TokioMutex::new(
                SocketService::init_socket(socket_name).await,
            )),
            event_bus,
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

    fn process_message(buffer: Vec<u8>, event_bus: Arc<Mutex<EventBus>>) {
        if let Ok(string) = String::from_utf8(buffer) {
            let msg: SocketMessage = serde_json::from_str(
                // remove any additional zeros from the buffer
                string.trim_end_matches(char::from(0)),
            )
            .unwrap();

            event_bus.lock().unwrap().publish(
                EVENT_TOPIC,
                EventBusMessage::new(
                    &msg.title,
                    EventType::Socket,
                    Some(vec![(EventFieldType::Description, msg.status.into_bytes())]),
                )
                .format_bytes(),
            );
        }
    }

    // TODO: needs to handle errors / bad input / kick out clients
    async fn listen_on_socket(
        listener: Arc<TokioMutex<UnixListener>>,
        event_bus: Arc<Mutex<EventBus>>,
    ) {
        while let Ok((mut stream, _)) = listener.lock().await.accept().await {
            let mut buffer = vec![0u8; 1024];
            {
                let event_bus = Arc::clone(&event_bus);
                tokio::spawn(async move {
                    match stream.read(&mut buffer).await {
                        Err(err) => println!("err: {err}"),
                        Ok(_) => SocketService::process_message(buffer, event_bus),
                    }
                });
            }
        }
    }
}

impl Runnable for SocketService {
    fn run(&self) {
        let listener = Arc::clone(&self.listener);
        let event_bus = Arc::clone(&self.event_bus);
        tokio::spawn(async move {
            SocketService::listen_on_socket(listener, event_bus).await;
        });
    }
}
