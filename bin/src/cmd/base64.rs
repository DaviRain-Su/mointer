use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use clap::Parser;

#[derive(Parser, Debug)]
pub enum Base64 {
    #[command(name = "encode", about = "Encode base64 string")]
    Encode {
        #[arg(short, long)]
        input: String,
    },
    #[command(name = "decode", about = "Decode base64 string")]
    Decode {
        #[arg(short, long)]
        input: String,
    },
}

impl Base64 {
    pub fn run(&self) -> anyhow::Result<()> {
        match self {
            Base64::Encode { input } => {
                let result = URL_SAFE.encode(input);
                println!("{}", result);
            }
            Base64::Decode { input } => {
                let result = URL_SAFE.decode(input)?;
                println!("{}", String::from_utf8(result)?);
            }
        }
        Ok(())
    }
}
