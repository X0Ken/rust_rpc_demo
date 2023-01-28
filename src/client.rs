use common::rpc;


fn hello() -> String {
    return String::from("Hello, world!");
}


fn main() {
  let mut client = rpc::RPCClient::connect(String::from("127.0.0.1:8080"));
  let hello_fn: rpc::RPCFn = hello;
  client.fns.insert(String::from("hello"), hello_fn);
  client.dispatch();
}
