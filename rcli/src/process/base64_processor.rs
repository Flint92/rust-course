use crate::cli::base64_opts::{Base64DecoderOpts, Base64EncoderOpts};
use base64::engine::GeneralPurpose;
use base64::engine::general_purpose::PAD;
use base64::{Engine, alphabet};

const URL_SAFE_NO_PAD_ENGINE: GeneralPurpose = GeneralPurpose::new(&alphabet::URL_SAFE, PAD);

pub fn base64_encode(opts: Base64EncoderOpts) -> anyhow::Result<()> {
    let input = opts.input.as_bytes();
    let output = URL_SAFE_NO_PAD_ENGINE.encode(input);
    println!("{}", output);
    Ok(())
}

pub fn base64_decode(opts: Base64DecoderOpts) -> anyhow::Result<()> {
    let input = opts.input.as_bytes();
    let output = URL_SAFE_NO_PAD_ENGINE.decode(input)?;
    let output = String::from_utf8(output)?;
    println!("{}", output);
    Ok(())
}
