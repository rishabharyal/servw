use std::net::{TcpListener, TcpStream};
use std::io::Write;
use std::path::Path;
use std::process::exit;

use servw::config::Config;
use servw::http_validator::HttpValidator;
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
    let listener = TcpListener::bind("127.0.0.1:".to_string() + port)?;
    let alb_type = config.lb_algo();

    if alb_type == "off" {
        println!("Load balancing is disabled. We will use cgi pass instead.");
        for stream in listener.incoming() {
            std::thread::spawn(move || {
                let cgi_handler: dyn Handler = CgiHandler::new();
                if let Err(e) = handle_connection(stream.unwrap(), cgi_handler) {
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


    for stream in listener.incoming() {
        // handle in a thread so that we can keep listening for more connections

        std::thread::spawn(move |lb| {
            let handler: dyn Handler = ServerHandler::new(lb);
            if let Err(e) = handle_connection(stream.unwrap(), handler) {
                println!("Connection error: {}", e);
            }
        });
    }

    Ok(())
}

fn handle_connection(stream: TcpStream, handler: Box<dyn Handler>) -> std::io::Result<()> {
    handler.handle(stream);
    Ok(())
}
