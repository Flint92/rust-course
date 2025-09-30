use clap::Parser;

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Encode base64")]
    Encoder(Base64EncoderOpts),
    #[command(name = "decode", about = "Decode base64")]
    Decoder(Base64DecoderOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncoderOpts {
    #[arg(short, long)]
    pub input: String,
}

#[derive(Debug, Parser)]
pub struct Base64DecoderOpts {
    #[arg(short, long)]
    pub input: String,
}
