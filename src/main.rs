use std::env;
use std::io::{self, BufRead, Write};
use std::net::TcpStream;
use std::str::FromStr;

use native_tls::{TlsConnector, TlsStream};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("Usage: pop3_client <hostname> <port> <username>");
        return Ok(());
    }

    let hostname = &args[1];
    let port = u16::from_str(&args[2])?;
    let username = &args[3];

    // Connect to the server
    let mut stream = if port == 995 {
        // Use SSL if the port is 995 (typical for POP3S)
        let ssl = TlsConnector::new()?;
        let stream = TcpStream::connect((hostname, port))?;
        let stream = ssl.connect(hostname, stream)?;
        stream
    } else {
        TcpStream::connect((hostname, port))?
    };

    // Read the greeting message
    let mut reader = io::BufReader::new(&stream);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    println!("{}", line);

    // Send the USER command
    writeln!(stream, "USER {}", username)?;
    line.clear();
    reader.read_line(&mut line)?;
    println!("{}", line);

    // Send the PASS command
    print!("Enter password: ");
    io::stdout().flush()?;
    let mut password = String::new();
    io::stdin().read_line(&mut password)?;
    writeln!(stream, "PASS {}", password.trim())?;
    line.clear();
    reader.read_line(&mut line)?;
    println!("{}", line);

    // Send the QUIT command
    writeln!(stream, "QUIT")?;
    line.clear();
    reader.read_line(&mut line)?;
    println!("{}", line);

    Ok(())
}
