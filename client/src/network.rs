use fossil_shared::packet::Packet;
use std::io::Write;
use tokio::io;
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::BufReader;
use tokio::net::TcpStream;
use tokio::sync::mpsc::Receiver;

pub fn describe_packet(packet: &Packet) -> String {
    match packet {
        Packet::Error(error) => {
            format!("\n\x1b[31mServer Error: {}\x1b[0m", error)
        }
        Packet::Message { user, content } => {
            format!("\n\x1b[32m{}: {}\x1b[0m", user, content)
        }
        other => format!("Received: {:?}", other),
    }
}

pub fn parse_packet_line(line: &str) -> Result<Packet, serde_json::Error> {
    serde_json::from_str::<Packet>(line.trim())
}

pub async fn send_packet_line<W: AsyncWriteExt + Unpin>(
    writer: &mut W,
    packet: &Packet,
) -> io::Result<()> {
    let json = serde_json::to_string(packet).map_err(io::Error::other)?;
    writer.write_all(json.as_bytes()).await?;
    writer.write_all(b"\n").await?;
    writer.flush().await?;
    Ok(())
}

pub async fn run(mut _rx: Receiver<String>) {
    println!("Listener running.");
    let stream = match TcpStream::connect("fossil.simarpreetsingh.org:7878").await {
        Ok(s) => s,
        Err(_) => {
            eprintln!("Failed to connect to server");
            return;
        }
    };

    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);

    let stdin = io::stdin();
    let mut stdin = BufReader::new(stdin);

    let _handle = tokio::spawn(async move {
        let mut line = String::new();

        loop {
            line.clear();

            match reader.read_line(&mut line).await {
                Ok(0) => {
                    println!("Server disconnected.");
                    break;
                }
                Ok(_) => match parse_packet_line(&line) {
                    Ok(packet) => {
                        println!("{}", describe_packet(&packet));
                        std::process::exit(1);
                    }
                    Err(e) => {
                        eprintln!("Invalid packet: {}", e);
                    }
                },
                Err(e) => {
                    eprintln!("Read error: {}", e);
                    break;
                }
            }
        }
    });

    let mut name = String::new();

    print!("Enter your name (or !exit to leave chat): ");
    std::io::stdout().flush().unwrap();

    stdin.read_line(&mut name).await.unwrap();

    name = name.trim().to_string();

    if name.to_lowercase() == "!exit" {
        println!("Exiting...");
        return;
    }

    let packet_join = Packet::Join(name.to_string());
    if send_packet_line(&mut writer, &packet_join).await.is_err() {
        eprintln!("Failed to send join packet");
        return;
    }

    loop {
        // Repeatedly asks user for their message, sends packet to server
        let mut content = String::new();
        print!("Message (or !exit to leave chat): ");
        std::io::stdout().flush().unwrap();
        stdin.read_line(&mut content).await.unwrap();
        let content = content.trim();
        if content.to_lowercase() == "!exit" {
            println!("Exiting...");
            let packet_leave = Packet::Leave(name.to_string());
            if send_packet_line(&mut writer, &packet_leave).await.is_err() {
                eprintln!("Failed to send leave packet");
            }
            return;
        }
        if content.is_empty() {
            println!("Message cannot be empty!");
            continue;
        }
        println!("Sending message from {}: '{}'", name, content);

        let packet_send = Packet::Message {
            user: name.to_string(),
            content: content.to_string(),
        };
        if send_packet_line(&mut writer, &packet_send).await.is_err() {
            eprintln!("Failed to send message packet");
            break;
        }
    }
}
