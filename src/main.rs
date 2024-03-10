mod cli;
mod core;
mod server;

fn main() -> std::io::Result<()> {
    server::start()
}
