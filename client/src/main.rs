mod cryptography;
mod images;
use image::{GenericImageView, imageops::FilterType};
use images::save_image;
use std::{env, fs::File, io::Read};
use base64::{Engine as _, engine::general_purpose};
use clap::{Args, Parser, Subcommand};
use eyre::Result;

use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a new set of encryption keys
    GenerateKeys,
    
    /// Process and encrypt an image
    ProcessImage(ProcessImageArgs),
    
    /// Decrypt an encrypted image
    DecryptImage(DecryptImageArgs),
}

#[derive(Args)]
struct ProcessImageArgs {
    /// Path to the encryption keys
    #[arg(long)]
    keys: PathBuf,

    /// Path to the image
    image: PathBuf
}

#[derive(Args)]
struct DecryptImageArgs {
    /// Path to the encryption keys
    #[arg(long)]
    keys: PathBuf,
    
    /// The path to the encrypted image
    encrypted_image: PathBuf,

    /// Path to save image at
    #[arg(long)]
    save_path: PathBuf
}

fn main() -> Result<()>{
    let cli = Cli::parse();

    match &cli.command {
        Commands::GenerateKeys => {
            generate_keys()?;
        }
        Commands::ProcessImage(args) => {
            process_image(&args.keys, &args.image)?;
        }
        Commands::DecryptImage(args) => {
            decrypt_image(&args.keys, &args.encrypted_image, &args.save_path)?;
        }
    }

    Ok(())
}

// Mock functions - to be implemented later
fn generate_keys() -> Result<()>{
    // This function will generate encryption keys
    let keys = cryptography::Keys::new();
    keys.save("keys.bin".to_string())?;
    println!("new keys stored to keys.bin");

    Ok(())
}

fn process_image(keys_path: &PathBuf, image: &PathBuf) -> Result<()>{
    let keys = cryptography::Keys::load(keys_path.to_string_lossy().to_string())?; 
    let image_arr = images::load_image(image.to_string_lossy().to_string())?;

    let enc = keys.enc_array(image_arr);
    let b64 = cryptography::encode_enc_image(enc)?;

    println!("{}", b64);
    // send b64 to task operator


    Ok(())
}

fn decrypt_image(keys_path: &PathBuf, encrypted_image: &PathBuf, image_path: &PathBuf) -> Result<()> {
    let keys = cryptography::Keys::load(keys_path.to_string_lossy().to_string())?; 


    let mut file = File::open(encrypted_image)?;
    let mut enc_b64 = Vec::<u8>::new();
    file.read_to_end(&mut enc_b64)?;
    
    let enc = cryptography::decode_enc_image(&String::from_utf8(enc_b64)?)?;
    let dec = keys.dec_array(enc);

    images::save_image(&dec, image_path.to_string_lossy().to_string())?;

    Ok(())
}