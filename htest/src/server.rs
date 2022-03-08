use std::error::Error;
use std::fs::remove_file;
use std::thread;
use std::os::unix::net::{UnixStream, UnixListener};


fn handle_client(stream: UnixStream) {
    // ...
    println!("new stream client connect")
}

pub fn startServer()->std::io::Result<()> {
    let path="/tmp/htest-socket";
    remove_file(path).map_err(|error|error.to_string());
    let listener = UnixListener::bind(path)?;

    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                /* connection succeeded */
                thread::spawn(|| handle_client(stream));
            }
            Err(err) => {
                /* connection failed */
                break;
            }
        }
    }
    
    Ok(())
}