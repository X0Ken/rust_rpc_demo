use std::io::{self, Write};
use std::thread;
use common::rpc;
use std::sync::{Arc, RwLock};
use std::collections::hash_map::HashMap;
use std::net::TcpStream;


fn cli_read(clients: Arc<RwLock<HashMap<String, TcpStream>>>){
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("read line failed");
        input.pop();
        let map = clients.read().expect("RwLock poisoned");
        let mut stream = map.get(&String::from("client")).unwrap();
        println!("call client {}", input);
        stream.write(&input.as_bytes()).expect("write failed");
    }
}

fn main() {
    let server = rpc::RPCServer::create(String::from("127.0.0.1:8080"));
    let write_clients = Arc::clone(&server.clients);
    thread::spawn(move || { cli_read(write_clients) });
    server.listen();

    println!("Hello, world!");
}
