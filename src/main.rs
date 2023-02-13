pub mod cli;

use clap::{Arg, Command};
// use cli;
use tokio::io;
use tokio::net::{TcpListener, TcpStream};
use tokio::select;

#[tokio::main]
async fn main() -> io::Result<()> {
    let cli = Command::new("")
        .version("0.1")
        .author("")
        .about("A simple tcp proxy")
        .subcommands([
            Command::new("local")
                .about("Controls configuration functionality")
                .arg(Arg::new("config_file")),
            Command::new("server").about("Controls debug functionality"),
        ])
        .get_matches();
    Ok(())
}
