use std::net::TcpListener;
use std::io::Result;

const PORT: u16 = 8080;

pub fn start() -> Result<()> {
    let listener = TcpListener::bind(format!("127.0.0.1:{PORT}"))?;

    println!("Server is listening on 127.0.0.1:{PORT}...");

    for stream in listener.incoming() {
        match stream {
            Ok(_) => {
                println!("New connection established!");
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpStream;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_server_start() {
        // Start the server in a new thread
        thread::spawn(|| {
            start().unwrap();
        });
        // Give the server some time to start
        thread::sleep(Duration::from_millis(500));
        
        // Attempt to connect to the server
        let result = TcpStream::connect(format!("127.0.0.1:{PORT}"));

        // If we successfully connect, the server works
        assert!(result.is_ok());
    }
}

