use std::io::{self, Write, Read};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread;
use std::collections::hash_map::{HashMap, Entry};
use std::sync::{Arc, RwLock};

fn handle_client(mut stream: TcpStream, clients: Arc<RwLock<HashMap<&str, TcpStream>>>) {
    println!("client incoming");
    {
        let mut map = clients.write().expect("RwLock poisoned");
        match map.entry("client") {
            Entry::Occupied(..) => (),
            Entry::Vacant(entry) => {
                println!("client insert");
                entry.insert(stream.try_clone().unwrap());
            }
        }
    }

    let mut buf = [0; 128];
    loop {
        // 读取内容
        let len = stream.read(&mut buf).unwrap();
        if len == 0 {
            println!("ok");
            break;
        }
        // 输出读取到的内容
        println!("read {} bytes: {:?}", len, str::from_utf8(&buf[..len]));
    }
}

fn cli_read(clients: Arc<RwLock<HashMap<&str, TcpStream>>>){
    let mut input = String::new();
    loop {
        let size = io::stdin().read_line(&mut input).expect("read line failed");
        let map = clients.read().expect("RwLock poisoned");
        let mut stream = map.get("client").unwrap();
        println!("client get");
        stream.write(&input.as_bytes()[..size]).expect("write failed");
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let clients = Arc::new(RwLock::new(HashMap::new()));
    let write_clients = Arc::clone(&clients);

    thread::spawn(move || { cli_read(write_clients) });

    // 对每一个连接开启一个线程进行处理
    for stream in listener.incoming() {
        let write_clients = Arc::clone(&clients);
        thread::spawn(move || {
            handle_client(stream.unwrap(), write_clients);
        });
    }
    println!("Hello, world!");
}
