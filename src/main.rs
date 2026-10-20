use clap::Parser;
use joaju_lirica::{escandir_verso, parse_lirica_block, render_painel, render_typst, ModoSaida};
use std::io::{self, Read};

#[derive(Parser, Debug)]
#[command(name = "joaju-lirica")]
#[command(author = "Lumen Pink <hi@lumen.pink>")]
#[command(version)]
#[command(
    about = "Deterministic poetic scansion and rhythm analysis engine for Portuguese verse",
    long_about = None
)]
struct Cli {
    /// Raw poem verse or text to analyze
    #[arg(short, long)]
    text: Option<String>,

    /// Output format as structured JSON
    #[arg(short, long)]
    json: bool,

    /// Output format as Typst code
    #[arg(short, long)]
    typst: bool,
}

fn main() {
    let cli = Cli::parse();

    let input = if let Some(t) = cli.text {
        t
    } else {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .expect("Failed to read from stdin");
        buffer
    };

    let trimmed = input.trim();
    if trimmed.is_empty() {
        eprintln!("Error: no input provided.");
        std::process::exit(1);
    }

    if trimmed.contains("%%") || trimmed.lines().count() > 1 {
        // Multi-line or directive block
        match parse_lirica_block(&input) {
            Ok((config, poema)) => {
                if cli.json || config.modo_saida == ModoSaida::Json {
                    let json_out = serde_json::to_string_pretty(&poema)
                        .expect("Failed to serialize poem to JSON");
                    println!("{}", json_out);
                } else if cli.typst || config.modo_saida == ModoSaida::Typst {
                    print!("{}", render_typst(&config, &poema));
                } else {
                    print!("{}", render_painel(&poema));
                }
            }
            Err(e) => {
                eprintln!("Error parsing lirica block: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        // Single verse line
        let escansao = escandir_verso(trimmed);
        if cli.json {
            let json_out = serde_json::to_string_pretty(&escansao)
                .expect("Failed to serialize scansion to JSON");
            println!("{}", json_out);
        } else {
            let partes: Vec<String> = escansao
                .silabas
                .iter()
                .map(|s| {
                    let txt = if s.tonica {
                        format!("[{}]", s.texto)
                    } else {
                        s.texto.clone()
                    };
                    if let Some(num) = s.numero_metrico {
                        format!("{}:{}", num, txt)
                    } else {
                        format!("·{}", txt)
                    }
                })
                .collect();

            let class_str = escansao.classificacao.as_deref().unwrap_or("Indeterminado");

            println!(
                "{} | Total: {} ({}) | Tônicas: {:?}",
                partes.join("-"),
                escansao.total_metricas,
                class_str,
                escansao.esquema_tonico
            );
        }
    }
}
