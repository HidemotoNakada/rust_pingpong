
use std::net::{SocketAddrV4, Ipv4Addr, TcpListener, TcpStream};
use std::io::{Read, Write, Error};
use std::time::SystemTime;
use std::thread;

fn server_loop(mut stream: TcpStream) -> Result<(), Error> {
    loop {
        let mut buf = [0u8;16];
        stream.read_exact(& mut buf)?;

        let duration_since_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let now = duration_since_epoch.as_nanos(); // u128

        let client_time = u128::from_ne_bytes(buf);
        if client_time == 0 {
            break;
        }
        println!("{:?}", now - client_time);
        let msg = (now).to_ne_bytes();
        stream.write(&msg).unwrap();
        stream.flush()?;
    }
    Ok(())
}



fn server(port: u16) -> Result<(), Error> {
    loop {
        let all = Ipv4Addr::new(0, 0, 0, 0);
        let socket = SocketAddrV4::new(all, port);
        let listener = TcpListener::bind(socket)?;
        let port = listener.local_addr()?;
        println!("Listening on {}, access this port to end the program", port);
        let (stream, addr) = listener.accept()?; //block  until requested
        println!("Connection received! {:?} is sending data.", addr);
        thread::spawn(move || {
            server_loop(stream).unwrap();
        });

//
    }
//    Ok(())
}

fn main()  -> Result<(), Error> {
    let port_str = std::env::args().nth(1).expect("no port given");
    println!("port is {:?}", port_str);
    let port: u16 = (&port_str).parse::<u16>().unwrap();
    server(port)
}