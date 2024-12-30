use std::{collections::HashMap, env::temp_dir, path::Path, sync::Arc};

use tokio::{fs, io::AsyncReadExt, net::UnixListener, sync::Mutex};

use crate::models::socket_message::SocketMessage;

pub struct SocketService {
    pub queue: Arc<Mutex<HashMap<String, String>>>,
    socket_name: String,
}

impl SocketService {
    pub fn new(socket_name: &str) -> Self {
        Self {
            queue: Arc::new(Mutex::new(HashMap::new())),
            socket_name: socket_name.to_string(),
        }
    }

    async fn init_socket(&self) -> UnixListener {
        let dir = temp_dir();
        let bind_path = dir.join(&self.socket_name);
        if Path::new(&bind_path).exists() {
            fs::remove_file(&bind_path)
                .await
                .expect("unable to remove old socket on init");
        }

        UnixListener::bind(&bind_path).expect("unable to initialize socket listener")
    }

    // TODO: needs to handle errors / bad input / kick out clients
    // FIXME: very nested
    pub async fn run(&self) {
        let listener = self.init_socket().await;
        let queue = Arc::clone(&self.queue);
        tokio::spawn(async move {
            while let Ok((mut stream, _)) = listener.accept().await {
                let mut buffer = vec![0u8; 1024];
                {
                    let queue = Arc::clone(&queue);
                    tokio::spawn(async move {
                        match stream.read(&mut buffer).await {
                            Ok(_) => {
                                if let Ok(string) = String::from_utf8(buffer) {
                                    let msg: SocketMessage = serde_json::from_str(
                                        // remove any additional zeros from the buffer
                                        string.trim_end_matches(char::from(0)),
                                    )
                                    .unwrap();
                                    queue.lock().await.insert(msg.title, msg.status);
                                }
                            }
                            Err(err) => println!("err: {err}"),
                        }
                    });
                }
            }
        });
    }
}
