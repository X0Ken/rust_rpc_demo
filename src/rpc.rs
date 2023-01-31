use std::net::{TcpListener, TcpStream};
use std::collections::hash_map::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;
use std::str;
use std::io::{Read, Write};
use serde::{Deserialize, Serialize};

pub type RPCFn = fn() -> String;

#[derive(Debug)]
pub struct RPCServer {
    pub addr: String,
    pub clients: Arc<RwLock<HashMap<String, TcpStream>>>,
}

#[derive(Debug)]
pub struct RPCClient {
    pub addr: String,
    pub stream: TcpStream,
    pub fns: HashMap<String, RPCFn>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MessageType {
    Request,
    Response,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: u64,
    pub mtype: MessageType,
    pub method: String,
    pub value: String,
}

impl Message {
    pub fn serialize(&self) -> String {
      return serde_json::to_string(&self).unwrap();
    }

    pub fn deserialize(data: &[u8]) -> Message {
      let m: Message = serde_json::from_slice(data).unwrap();
      return m;
    }
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

    pub fn call(&self, method: String) {
        let map = self.clients.read().expect("RwLock poisoned");
        let mut stream = map.get(&String::from("client")).unwrap();
        println!("call client {}", method);
        let msg = Message{
          id: 0,
          mtype: MessageType::Request,
          method: method,
          value: String::from(""),
        };
        stream.write(&msg.serialize().as_bytes()).expect("write failed");
    }
}

impl RPCClient {
    pub fn connect(addr: String) -> RPCClient{
        println!("try connect on {}", addr);
        let stream = TcpStream::connect(addr.as_str()).expect("connect failed");
        let fns = HashMap::new();
        RPCClient {
            addr,
            stream: stream,
            fns,
        }
    }

    pub fn dispatch(&self){
        loop {
          let mut stream = self.stream.try_clone().unwrap();
          let mut buf = [0; 128];
          let len = stream.read(&mut buf).unwrap();
          if len == 0 {
              println!("ok");
              break;
          }
          let msg = str::from_utf8(&buf[..len]);
          println!("read {} bytes: {:?}", len, msg);
          let msg = Message::deserialize(&buf[..len]);
          let rsp = match self.fns.get(&msg.method) {
              Some(fn_call) => fn_call(),
              None => String::from("undefined")
          };
          let msg = Message{
            id: msg.id,
            mtype: MessageType::Response,
            method: msg.method,
            value: rsp,
          };
          stream
            .write(&msg.serialize().as_bytes())
            .expect("write failed");
        }
    }

    pub fn call(&self, input: String) {
        let mut stream = self.stream.try_clone().unwrap();
        println!("call server {}", input);
        stream.write(&input.as_bytes()).expect("write failed");
    }
}
