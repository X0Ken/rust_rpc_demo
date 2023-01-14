use std::io::{self, Write, Read};
use std::net::TcpStream;
use std::str;
use std::thread;
use std::collections::hash_map::HashMap;


type RPCFn = fn() -> String;

fn hello() -> String {
    return String::from("Hello, world!");
}


fn tcp_read(mut stream: TcpStream, fns: HashMap<&str, RPCFn>) {
  loop {
    let mut buf = [0; 128];
    let len = stream.read(&mut buf).unwrap();
    if len == 0 {
        println!("ok");
        break;
    }
    println!("read {} bytes: {:?}", len, str::from_utf8(&buf[..len]));
    let fn_name = str::from_utf8(&buf[..len-1]).unwrap();
    let rsp = match fns.get(&fn_name) {
        Some(fn_call) => fn_call(),
        None => String::from("undefined")
    };
    stream
      .write(&rsp.as_bytes())
      .expect("write failed");
  }
}

fn main() {
  let mut stream = TcpStream::connect("127.0.0.1:8080").expect("connect failed");
  let reader = stream.try_clone().unwrap();
  let mut fns = HashMap::new();
  let hello_fn: RPCFn = hello;
  fns.insert("hello", hello_fn);
  thread::spawn(move || { tcp_read(reader, fns) });


  loop {
    let mut input = String::new();
    let size = io::stdin().read_line(&mut input).expect("read line failed");

    stream
      .write(&input.as_bytes()[..size])
      .expect("write failed");
  }
}
