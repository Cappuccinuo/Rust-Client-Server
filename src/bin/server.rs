use std::thread;
use std::net::{TcpListener, SocketAddr};
use std::io;

fn main() -> io::Result<()> {
    /* TcpListener: https://doc.rust-lang.org/std/net/struct.TcpListener.html
     * Create a TCP listener bound to 127.0.0.1:8888,
     * if that fails, create a TCP listener bound to 127.0.0.1:8889
     */
    let addrs = [
        SocketAddr::from(([127, 0, 0, 1], 8888)),
        SocketAddr::from(([127, 0, 0, 1], 8889))
    ];
    let listener = TcpListener::bind(&addrs[..]).unwrap();
    let port = listener.local_addr().unwrap().port();
    println!("Server listening on port {}", port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // The socket address of the remote peer of this TCP connection
                let peer_address = stream.peer_addr().unwrap();
                println!("New connection: {}", peer_address);
                drop(peer_address);
                thread::spawn({
                    // https://doc.rust-lang.org/edition-guide/rust-2018/error-handling-and-panics/the-question-mark-operator-for-easier-error-handling.html
                    // To-Do
                    handle_client(stream?);
                });
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }

    // free the resources, https://doc.rust-lang.org/rust-by-example/trait/drop.html
    drop(addrs);
    drop(listener);
    drop(port);

    Ok(())
}