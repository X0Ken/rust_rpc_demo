use std::io;
use std::thread;
use common::rpc;
use std::sync::Arc;


fn cli_read(server: Arc<rpc::RPCServer>){
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("read line failed");
        input.pop();
        server.call(input);
    }
}

fn main() {
    let server = rpc::RPCServer::create(String::from("127.0.0.1:8080"));
    let server_clone = Arc::new(server);
    let server_local = Arc::clone(&server_clone);
    thread::spawn(move || { cli_read(server_clone) });
    server_local.listen();

    println!("Hello, world!");
}
