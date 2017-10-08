use std::io::Write;
use std::net::TcpListener;
use std::thread;
use std::time;

fn main() {
    let child = thread::spawn(tcp_listener);

    println!("Spawned child");
    thread::sleep(time::Duration::from_secs(10));
    println!("exiting");

    child.join().unwrap();
}

fn tcp_listener() {
    let listener = TcpListener::bind("127.0.0.1:12345");

    println!("Listening started on port 12345");

    let now = time::SystemTime::now();
    
    match listener {
        Ok(listener) => 
            for stream in listener.incoming() {
                match now.elapsed() {
                    Ok(elapsed) => {
                        if elapsed.as_secs() > 15 {
                            break;
                        }
                    },
                    Err(e) => {
                        println!("Error: {:?}", e);
                    }
                }
                thread::spawn(|| {
                   let mut stream = stream.unwrap();
                   stream.write(b"hello world\r\n").unwrap();
                });
            },
        Err(e) => println!("Got error: {:?}", e)
    }   
}
