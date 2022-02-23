use std::thread;
use std::os::unix::net::{UnixStream, UnixListener};


fn handle_client(stream: UnixStream) {
    // ...
    println!("new stream client connect")
}

pub fn startServer()->std::io::Result<()> {
    let listener = UnixListener::bind("/tmp/htest-socket")?;

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