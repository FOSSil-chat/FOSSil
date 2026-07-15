use fossil_shared::packet::Packet;

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    io::Write,
};

use tokio::{
    io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
    sync::mpsc::{self, Receiver},
};

pub fn disconnect() {
    std::process::exit(1);
}

fn get_color_for_user(username: &str) -> u8 {
    let mut hasher = DefaultHasher::new();
    username.hash(&mut hasher);

    // ANSI 256-colour range: 16-231
    16 + (hasher.finish() % 216) as u8
}

pub fn describe_packet(packet: &Packet) -> String {
    match packet {
        Packet::Error(error) => {
            format!("\x1b[31m[Error] {}\x1b[0m", error)
        }

        Packet::Message { user, content } => {
            let color = get_color_for_user(user);

            format!("\x1b[38;5;{}m{}: {}\x1b[0m", color, user, content)
        }

        _ => String::new(),
    }
}

pub fn parse_packet_line(line: &str) -> Result<Packet, serde_json::Error> {
    serde_json::from_str(line.trim())
}

pub async fn send_packet_line<W>(
    writer: &mut W,
    packet: &Packet,
) -> Result<(), Box<dyn std::error::Error>>
where
    W: AsyncWriteExt + Unpin,
{
    let json = serde_json::to_string(packet)?;

    writer.write_all(json.as_bytes()).await?;
    writer.write_all(b"\n").await?;
    writer.flush().await?;

    Ok(())
}

pub async fn run(mut _rx: Receiver<String>) {
    let stream = match TcpStream::connect("fossil.simarpreetsingh.org:7878").await {
        Ok(stream) => stream,

        Err(_) => {
            eprintln!("Failed to connect to server");
            return;
        }
    };

    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);

    let (packet_tx, mut packet_rx) = mpsc::channel::<Packet>(100);

    tokio::spawn(async move {
        let mut line = String::new();

        loop {
            line.clear();

            match reader.read_line(&mut line).await {
                Ok(0) => break,

                Ok(_) => {
                    if let Ok(packet) = parse_packet_line(&line) {
                        let _ = packet_tx.send(packet).await;
                    }
                }

                Err(_) => break,
            }
        }
    });

    let (input_tx, mut input_rx) = mpsc::channel::<String>(100);

    tokio::spawn(async move {
        let stdin = io::stdin();
        let mut stdin = BufReader::new(stdin);

        loop {
            let mut input = String::new();

            if stdin.read_line(&mut input).await.is_err() {
                break;
            }

            let _ = input_tx.send(input.trim().to_string()).await;
        }
    });

    let name = loop {
        print!("Enter your name (or !exit to leave chat): ");
        std::io::stdout().flush().unwrap();

        let username = match input_rx.recv().await {
            Some(name) => name,

            None => return,
        };

        if username == "!exit" {
            std::process::exit(0);
        }

        let packet = Packet::Join(username.clone());

        if send_packet_line(&mut writer, &packet).await.is_err() {
            eprintln!("Failed to send join packet");
            return;
        }

        match tokio::time::timeout(std::time::Duration::from_secs(2), packet_rx.recv()).await {
            Ok(Some(Packet::Error(error))) => {
                println!("\x1b[31m[Error] {}\x1b[0m", error);
            }

            _ => break username,
        }
    };

    println!("\nJoined chat as {}\n", name);

    loop {
        print!("Message (or !exit to leave chat): ");
        std::io::stdout().flush().unwrap();

        tokio::select! {
            Some(input) = input_rx.recv() => {
                if input == "!exit" {
                    let _ = send_packet_line(
                        &mut writer,
                        &Packet::Leave(name.clone()),
                    )
                    .await;
                    std::process::exit(0);
                }


                if input.is_empty() {
                    continue;
                }


                let packet = Packet::Message {
                    user: name.clone(),
                    content: input,
                };


                if send_packet_line(&mut writer, &packet)
                    .await
                    .is_err()
                {
                    break;
                }
            }


            Some(packet) = packet_rx.recv() => {
                let message = describe_packet(&packet);

                if !message.is_empty() {
                    println!("\n{}", message);
                }
            }
        }
    }
}
