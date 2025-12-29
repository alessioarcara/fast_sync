use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    /// Directory sorgente
    #[arg(short, long)]
    pub source: String,

    /// Directory destinazione
    #[arg(short, long)]
    pub dest: String,
}
