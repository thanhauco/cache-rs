use clap::Parser;
#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    port: u16,
}
fn main() {
    let args = Cli::parse();
}