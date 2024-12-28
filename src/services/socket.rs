use std::{env::temp_dir, path::Path, sync::Arc};

use tokio::{fs, io::AsyncReadExt, net::UnixListener, sync::Mutex};

pub struct SocketService {
    pub queue: Arc<Mutex<Vec<String>>>,
    socket_name: String,
}

impl SocketService {
    pub fn new(socket_name: &str) -> Self {
        Self {
            queue: Arc::new(Mutex::new(vec![])),
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
    pub async fn run(&self) {
        let listener = self.init_socket().await;

        while let Ok((mut stream, _)) = listener.accept().await {
            let mut buffer = vec![0u8; 1024];
            {
                let queue = Arc::clone(&self.queue);
                tokio::spawn(async move {
                    println!("new thread spawned");
                    match stream.read(&mut buffer).await {
                        Ok(n) => {
                            println!("read {n} bytes");
                            if let Ok(string) = String::from_utf8(buffer) {
                                queue.lock().await.push(string);
                            }
                        }
                        Err(err) => println!("err: {err}"),
                    }
                });
            }
        }
    }
}
