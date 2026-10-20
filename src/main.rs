use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "joaju-lirica")]
#[command(author = "Lumen Pink <hi@lumen.pink>")]
#[command(version)]
#[command(about = "Deterministic poetic scansion and rhythm analysis engine for Portuguese verse", long_about = None)]
struct Cli {
    /// Raw poem verse or text to analyze
    #[arg(short, long)]
    text: Option<String>,

    /// Output format as structured JSON
    #[arg(short, long)]
    json: bool,
}

fn main() {
    let _cli = Cli::parse();
    println!("joaju-lirica v{}", env!("CARGO_PKG_VERSION"));
}
