use common::rpc;
use std::sync::Arc;
use std::io;
use std::thread;


fn hello() -> String {
    return String::from("Hello, world!");
}

fn cli_read(client: Arc<rpc::RPCClient>){
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("read line failed");
        input.pop();
        client.call(input);
    }
}


fn main() {
    let mut client = rpc::RPCClient::connect(String::from("127.0.0.1:8080"));
    let hello_fn: rpc::RPCFn = hello;
    client.insert(String::from("hello"), hello_fn);
    let client_clone = Arc::new(client);
    let client_local = Arc::clone(&client_clone);
    thread::spawn(move || { cli_read(client_clone) });
    client_local.dispatch();
}
