use std::env;
use std::net::UdpSocket;
use std::str;
fn start_server()-> std::io::Result<()>{
    let socket = UdpSocket::bind("127.0.0.1:34254").expect("Failed creating server");
    let mut buf = [0;50];
    let (amt,src) = socket.recv_from(&mut buf).expect("Failed trying to received datagram");
    let buf = &mut buf[..amt];
    println!("data received: {:?}", str::from_utf8(buf));
    buf.reverse();

    socket.send_to(buf, &src)?;
    Ok(())
 }

fn start_client() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:3400").expect("couldn't bind to address");
    let message = String::from("👹👹");
    socket.connect("127.0.0.1:34254").expect("connect function failed");

    socket.send((message).as_bytes()).expect("Failed sending message");
    Ok(())
}
fn main() -> std::io::Result<()> {
    let args:Vec<String> = env::args().collect();
    let mode = &args[1];
    match mode.as_str() {
        "server"=> start_server(),
        "client" => start_client(),
        _ => Ok(()),
    };
    Ok(())
}

