
use std::net::{TcpStream};
use std::io::{Read, Write, Error};
use std::time::{SystemTime, Duration};
use std::thread;


fn time_in_u128() -> u128 {
    let duration_since_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    duration_since_epoch.as_nanos()
}

fn send(mut stream: & TcpStream, val: u128) -> Result<(), Error> {
    let msg = val.to_ne_bytes();

    stream.write(&msg).unwrap();
    stream.flush()?;
    Ok(())
}

fn client_loop(mut stream: TcpStream, interval: u64) -> Result<(), Error> {
    loop {
        let before = time_in_u128();
        send(& stream, before)?;

        let mut buf = [0u8;16];
        stream.read_exact(& mut buf)?;
        let server_time = u128::from_ne_bytes(buf);
    
        let after = time_in_u128();
        let t1:i128 = (server_time as i128) - (before as i128);
        let t2:i128 = (after as i128) - (server_time as i128);
        let diff: f64 = ((t2-t1) as f64) / 2.0;
    
        println!("{:.2} {}, {}, difference estimation = {}", 
                (before as f64) / 1000_000_000.0, t1, t2, diff); 
        if interval == 0 {
            send(& stream, 0)?;
            break;
        } else {
            thread::sleep(Duration::from_secs(interval));
        }
    }
    Ok(())
}

// interval : in sec. if it is 0, one shot.
fn client(host: & String, port: u16, interval: u64) -> Result<(), Error> {
    let connect_str = format!("{}:{}", host, port);
    match TcpStream::connect(connect_str) {
        Ok(stream) => {
            client_loop(stream, interval)?
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
    let interval = match std::env::args().nth(3) {
        Some(str) => {
            str.parse::<u64>().unwrap()
        },
        None => {
            0
        }
    };
    
    println!("conecting to host/port is {:?}/{:?}", host_str, port_str);
    let port: u16 = (&port_str).parse::<u16>().unwrap();
    client(& host_str, port, interval)?;
    Ok(())
}