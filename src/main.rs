use clap::{Parser, Subcommand};

mod msg;

/// Simple program to greet a person
#[derive(clap::Parser, Debug)]
#[command(
    version,
    about,
    name = "mls",
    bin_name = "mls",
    rename_all = "kebab-case"
)]
struct Mls {
    #[arg(default_value_t = 20, short, long)]
    draft: u8,
    #[clap(subcommand)]
    cmd: Commands,
}

#[derive(Debug, Subcommand)]
#[allow(clippy::enum_variant_names)]
enum Commands {
    Msg,
}

fn main() {
    let mls: Mls = Mls::parse();

    match mls.cmd {
        Commands::Msg {} => msg::decode_msg(mls.draft),
    }
}

pub fn read_stdin() -> String {
    use std::io::BufRead as _;

    let stdin = std::io::stdin();
    let mut result = vec![];
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        result.push(line);
    }
    result.join("")
}
