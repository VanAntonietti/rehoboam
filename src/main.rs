mod app;
mod modules;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    app::run()
}
