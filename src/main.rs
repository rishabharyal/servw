use std::net::{TcpListener, TcpStream};
use std::io::Write;
use std::path::Path;
use std::process::exit;

use servw::config::Config;
use servw::http_validator::HttpValidator;
use servw::lbs::{LeastConn, LoadBalancer, None, RoundRobin};

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

    // Based on the configuration, we will create instances of the load balancers, and handlers
    //
    let alb_type = config.lb_algo();

    if alb_type == "off" {
        println!("Load balancing is disabled. We will use cgi pass instead.");
        return Ok(());
    }

    let mut lb: Box<dyn LoadBalancer> = match alb_type {
        "none" => Box::new(None::new(config.servers())),
        "roundrobin" => Box::new(RoundRobin::new(config.servers())),
        "leastconn" => Box::new(LeastConn::new(config.servers())),
        _ => {
            println!("Error: Invalid load balancing algorithm");
            exit(1);
        }
    };

    let port = config.listen();
    let listener = TcpListener::bind("127.0.0.1:".to_string() + port)?;

    for stream in listener.incoming() {
        // handle in a thread so that we can keep listening for more connections
        std::thread::spawn(move || {
            if let Err(e) = handle_connection(stream.unwrap()) {
                println!("Connection error: {}", e);
            }
        });
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> { 
    // validate if the request is a valid HTTP request
    let mut http_validator = HttpValidator::new(&mut stream);
    if !http_validator.validate() {
        stream.write(b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\n Invalid HTTP request deteced!")?;
        return Ok(());
    }

    // send a response back to the client
    stream.write(b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\n Welcome to ServW")?;
    println!("Connection terminated");

    Ok(())
}
