use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, BufReader, AsyncBufReadExt};

mod sds;

#[tokio::main]
async fn main() {
    let port = 6380;

    let listener = TcpListener::bind(&format!("127.0.0.1:{}", port)).await.unwrap();

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let (reader, _writer) = socket.split();

            let mut buf_reader = BufReader::new(reader);

            loop {
                let mut line = String::new();

                let len = buf_reader.read_line(&mut line).await.unwrap();

                if len == 0 {
                    continue;
                }

                println!("{:?}", line);
            }
        });
    }
}

fn process(socket: TcpStream) {

}
