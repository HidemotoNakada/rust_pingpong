
use std::net::{TcpStream};
use std::io::{Read, Write, Error};
use std::time::{SystemTime};

fn time_in_u128() -> u128 {
    let duration_since_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    duration_since_epoch.as_nanos()
}


fn client(host: & String, port: u16) -> Result<(), Error> {
    let connect_str = format!("{}:{}", host, port);
    match TcpStream::connect(connect_str) {
        Ok(mut stream) => {
            let before = time_in_u128();
            let msg = before.to_ne_bytes();

            stream.write(&msg).unwrap();
            stream.flush()?;

            let mut buf = [0u8;16];
            stream.read_exact(& mut buf)?;
            let server_time = u128::from_ne_bytes(buf);

            let after = time_in_u128();
            let t1:i128 = (server_time as i128) - (before as i128);
            let t2:i128 = (after as i128) - (server_time as i128);
            let diff: f64 = ((t2-t1) as f64) / 2.0;

            println!("{}, {}, difference estimation = {}", t1, t2, diff); 
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }    


    Ok(())
}

fn main()  -> Result<(), Error> {
    let host_str = std::env::args().nth(1).expect("no host given");
    let port_str = std::env::args().nth(2).expect("no port given");
    println!("conecting to host/port is {:?}/{:?}", host_str, port_str);
    let port: u16 = (&port_str).parse::<u16>().unwrap();
    client(& host_str, port)
}