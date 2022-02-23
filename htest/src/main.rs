mod server;
mod client;
use std::{sync::mpsc::{self, Receiver}, thread::{self}, time::Duration};

use crate::server::{startServer};
fn main() {
    let (tx, rx) = mpsc::channel();
    let handler=thread::spawn(|| {
        let result=startServer();
        if result.is_ok()   {
            println!("Hello, world!");
            
        } else{
            println!("result:{:?}",result);
        }
    });

    let clientHandler=thread::spawn(||{
        for i in 0..5  {
            thread::sleep(Duration::from_millis(1000));
            client::startClient();
        }
        
    });

    let received:Receiver<String> = rx.recv().unwrap(); 
    println!("Got: {:?}", received);

    
    
}
