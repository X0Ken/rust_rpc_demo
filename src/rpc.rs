use std::net::{TcpListener, TcpStream};
use std::collections::hash_map::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;
use std::str;
use std::io::Read;


#[derive(Debug)]
pub struct RPCServer {
    pub addr: String,
    pub clients: Arc<RwLock<HashMap<String, TcpStream>>>,
}

#[derive(Debug)]
pub struct RPCClient {
    pub addr: String,
    pub stream: Option<TcpStream>,
}

fn handle_client(mut stream: TcpStream, clients: Arc<RwLock<HashMap<String, TcpStream>>>) {
    println!("client incoming");
    {
        let mut map = clients.write().expect("RwLock poisoned");
        map.insert(String::from("client"), stream.try_clone().unwrap());
    }

    loop {
        // 读取内容
        let mut buf = [0; 128];
        let len = stream.read(&mut buf).unwrap();
        if len == 0 {
            println!("ok");
            break;
        }
        // 输出读取到的内容
        println!("read {} bytes: {:?}", len, str::from_utf8(&buf[..len]));
    }
}

impl RPCServer {
    pub fn create(addr: String) -> RPCServer {
        let clients = Arc::new(RwLock::new(HashMap::new()));
        RPCServer {
            addr,
            clients,
        }
    }

    pub fn listen(&self) {
        println!("try bind on {}", self.addr);
        let listener = TcpListener::bind(self.addr.as_str()).unwrap();
        for stream in listener.incoming() {
            let write_clients = Arc::clone(&self.clients);
            thread::spawn(move || {
                handle_client(stream.unwrap(), write_clients);
            });
        }
    }
}

impl RPCClient {
    pub fn create(addr: String) -> RPCClient{
        RPCClient {
            addr,
            stream: None,
        }
    }

    pub fn connect(&mut self) {
        println!("try connect on {}", self.addr);
        self.stream = Some(TcpStream::connect(self.addr.as_str()).expect("connect failed"));
    }
}
