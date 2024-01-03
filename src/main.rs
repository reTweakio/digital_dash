use tokio::net::UdpSocket;

#[tokio::main]
async fn main() {
    let local_addr = "10.0.0.58:8080";
    let socket = UdpSocket::bind(&local_addr).await.expect("Failed to bind to address");

    println!("Listening on: {}", local_addr);

    loop {
        let mut buf: Vec<u8> = vec![0; 1024];
        let (len, src) = socket.recv_from(&mut buf).await.expect("Failed to receive data");

        println!("Received {} bytes from {}: {:?}", len, src, &buf[..len]);
    }
}
