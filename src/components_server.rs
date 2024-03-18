// src/bin/server.rs
use std::os::unix::net::{UnixListener, UnixStream};
use std::io::{Read, Write};

fn server() -> anyhow::Result<()> {
    let socket_path = "mysocket";

    let unix_listener =
        UnixListener::bind(socket_path)?;

    // put the server logic in a loop to accept several connections
    loop {
        let (mut unix_stream, socket_address) = unix_listener
            .accept()?;
        handle_stream(unix_stream)?;
    }
    Ok(())
}

// src/bin/server.rs
fn handle_stream(mut unix_stream: UnixStream) -> anyhow::Result<()> {
    let mut message = String::new();
    unix_stream
        .read_to_string(&mut message)?;

    println!("We received this message: {}\nReplying...", message);

    unix_stream
        .write(b"I hear you!")?;

    Ok(())
}

