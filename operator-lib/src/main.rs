use std::{fs::File, io::{Read, Write}};
use eyre::Result;
use operator::process_encrypted_image;
use tfhe::{generate_keys, ConfigBuilder};

mod operator;

fn main() -> Result<()> {
    // generate dummy key
    let config = ConfigBuilder::default();
    let (_, srk) = generate_keys(config);

    let mut file = File::open("./enc.b64")?;
    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf)?;

    let res = process_encrypted_image(&String::from_utf8(buf)?, srk)?;

    let mut save_file = File::create("./proc.b64")?;
    save_file.write_all(res.as_bytes())?;

    Ok(())
}
