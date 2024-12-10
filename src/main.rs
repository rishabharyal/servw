use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::io::Write;
use servw::config::Config;
use servw::lbs::{LeastConn, LoadBalancer, None, RoundRobin};
use servw::handlers::{CgiHandler, Handler, ServerHandler};

fn main() -> std::io::Result<()> {

    let mut config = Config::new();

    match config.parse("http.conf") {
        Ok(_) => {},
        Err(e) => {
            println!("Config parsing error: {}", e);
            exit(1);
        }
    }

    // check if the root folder exists
    if !Path::new(&config.root()).exists() {
        println!("Error: Root folder does not exist");
        exit(1);
    }

    let port = config.listen();
    println!("Listening to port {}", port);
    let listener = TcpListener::bind("127.0.0.1:".to_string() + port)?;
    let alb_type = config.lb_algo();

    if alb_type == "off" {
        println!("Load balancing is disabled. We will use cgi pass instead.");
        for stream in listener.incoming() {
            std::thread::spawn(move || {
                let cgi_handler: Box<dyn Handler> = Box::new(CgiHandler::new());
                if let Err(e) = handle_connection(stream.as_ref().unwrap(), cgi_handler) {
                    println!("Connection error: {}", e);
                }
            });
        }
        return Ok(());
    }

    let lb: Box<dyn LoadBalancer> = match alb_type {
        "none" => Box::new(None::new(config.servers())),
        "roundrobin" => Box::new(RoundRobin::new(config.servers())),
        "leastconn" => Box::new(LeastConn::new(config.servers())),
        _ => {
            println!("Error: Invalid load balancing algorithm");
            exit(1);
        }
    };

    let mutexlb = Arc::new(Mutex::new(lb));
    println!("starting listening to the incoming requests");
    for stream in listener.incoming() {
        let mutexlb = mutexlb.clone();
        std::thread::spawn(move || {
            let handler: Box<dyn Handler> = Box::new(ServerHandler::new(mutexlb));
            let mut stream = stream.unwrap(); 
            match handle_connection(&stream, handler) {
                Ok(result) => {
                    // Create proper HTTP response with headers

                    // Single write with proper error handling
                    match stream.write_all(result.as_bytes()) {
                        Ok(_) => {
                            stream.flush().unwrap_or_default();
                        },
                        Err(e) => {
                            println!("Write error: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("Connection error: {}", e);
                    let error_response = format!(
                        "HTTP/1.1 500 Internal Server Error\r\n\
                            Content-Length: {}\r\n\
                            Content-Type: text/plain\r\n\
                            Connection: close\r\n\
                            \r\n\
                            {}", 
                        e.to_string().len(),
                        e
                    );
                    if let Err(write_err) = stream.write_all(error_response.as_bytes()) {
                        println!("Error writing error response: {}", write_err);
                    }
                    stream.flush().unwrap_or_default();
                }
            }
        });
    }
    Ok(())
}

fn handle_connection(stream: &TcpStream, handler: Box<dyn Handler>) -> Result<String, std::io::Error> {
    let result = handler.handle(stream);
    Ok(result)
}
