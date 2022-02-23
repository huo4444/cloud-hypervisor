use std::os::unix::net::UnixStream;
use std::io::prelude::*;

pub fn startClient()->std::io::Result<()> {
    let mut stream = UnixStream::connect("/tmp/htest-socket")?;
    stream.write_all(b"hello world")?;
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    println!("{}", response);
    Ok(())
}