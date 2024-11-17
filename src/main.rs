use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};


fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000")?;

    for stream in listener.incoming() {

        // handle in a thread so that we can keep listening for more connections
        println!("Incoming connection");
        std::thread::spawn(move || {
            if let Err(e) = handle_connection(stream.unwrap()) {
                println!("Connection error: {}", e);
            }
        });
    }

    Ok(())

}
fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> { 
    println!("Connection established");
    let mut buffer = [0;1024];
    stream.read(&mut buffer)?;
    let _data = String::from_utf8_lossy(&buffer);

    // send a response back to the client
    stream.write(b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\n")?;
    println!("Connection terminated");

    Ok(())
}
