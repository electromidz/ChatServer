use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    let (tx, mut rx) = broadcast::channel::<String>(10);
    loop {
        let (mut socket, _addr) = listener.accept().await.unwrap();

        let tx = tx.clone();
        let mut rx = tx.subscribe();
        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();

            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                let byte_read = reader.read_line(&mut line).await.unwrap();
                if byte_read == 0 {
                    break;
                }

                tx.send(line.clone()).unwrap();

                let msg = rx.recv().await.unwrap();

                writer.write_all(msg.as_bytes()).await.unwrap();
                line.clear();
            }
        });
    }
}
