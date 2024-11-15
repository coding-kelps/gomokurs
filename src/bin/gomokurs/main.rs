use gomokurs::inbound::NamedPipe;

#[tokio::main]
async fn main() {
    let res = NamedPipe::new("./fifo").await;
    if let Err(e) = res {
        eprintln!("error: {}", e);
    }

    loop {
    }
}
