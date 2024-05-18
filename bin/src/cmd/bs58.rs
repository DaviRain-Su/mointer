use clap::Parser;
use colored::*;

#[derive(Parser, Debug)]
pub enum Bs58 {
    #[command(name = "encode", about = "Encode bs58 string")]
    Encode {
        #[arg(short, long)]
        input: String,
    },
    #[command(name = "decode", about = "Decode bs58 string")]
    Decode {
        #[arg(short, long)]
        input: String,
    },
}

impl Bs58 {
    pub fn run(&self) -> anyhow::Result<()> {
        match self {
            Bs58::Encode { input } => {
                let result = bs58::encode(input).into_string();
                println!("{}", result.blue());
            }
            Bs58::Decode { input } => {
                let result = bs58::decode(input).into_vec()?;
                println!("{}", String::from_utf8(result)?.red());
            }
        }
        Ok(())
    }
}
