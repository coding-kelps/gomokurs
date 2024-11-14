use gomokurs::inbound::NamedPipe;

fn main() {
    if let Err(e) = NamedPipe::new("./fifo") {
        eprintln!("error: {}", e)
    }
}
