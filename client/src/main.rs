mod cryptography;
mod images;
use base64::{Engine as _, engine::general_purpose};
use clap::{Args, Parser, Subcommand};
use dotenv::dotenv;
use eyre::Result;
use futures::executor::block_on;
use image::{GenericImageView, imageops::FilterType};
use images::save_image;
use pinata_sdk::PinByJson;
use std::collections::HashMap;
use std::path::PathBuf;
use std::{env, fs::File, io::Read};
use tfhe::boolean::backward_compatibility::public_key;
use tokio;
use serde_json::{Map, Value};


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

    /// Fetch image from IPFS proof of task
    FetchImage(FetchImageArgs),
}

#[derive(Args)]
struct ProcessImageArgs {
    /// Path to the encryption keys
    #[arg(long)]
    keys: PathBuf,

    /// Path to the image
    image: PathBuf,

    /// URL for the ExecutionService
    #[arg(long, default_value = "http://localhost:4003/task")]
    execution_url: String
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
    save_path: PathBuf,
}

#[derive(Args)]
struct FetchImageArgs {
    /// Path to the encryption keys
    #[arg(long)]
    keys: PathBuf,

    /// IPFS Hash of the proof of task
    ipfs_hash: String,

    /// Path to save image at
    #[arg(long)]
    save_path: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let cli = Cli::parse();

    match &cli.command {
        Commands::GenerateKeys => {
            generate_keys()?;
        }
        Commands::ProcessImage(args) => {
            process_image(&args.keys, &args.image, &args.execution_url).await?;
        }
        Commands::DecryptImage(args) => {
            decrypt_image(&args.keys, &args.encrypted_image, &args.save_path)?;
        }
        Commands::FetchImage(args) => {
            fetch_image(&args.keys, &args.ipfs_hash, &args.save_path).await?;
        }
    }

    Ok(())
}

// Mock functions - to be implemented later
fn generate_keys() -> Result<()> {
    // This function will generate encryption keys
    let keys = cryptography::Keys::new();
    keys.save("keys.bin".to_string())?;
    println!("new keys stored to keys.bin");

    Ok(())
}

async fn process_image(
    keys_path: &PathBuf,
    image: &PathBuf,
    execution_service: &String,
) -> Result<()> {
    let keys = cryptography::Keys::load(keys_path.to_string_lossy().to_string())?;
    let image_arr = images::load_image(image.to_string_lossy().to_string())?;

    let enc = keys.enc_array(image_arr);
    let b64 = cryptography::encode_enc_image(enc)?;

    let public_key = env::var("PINATA_API_KEY")?;
    let secret_key = env::var("PINATA_SECRET_API_KEY")?;

    let pinata = pinata_sdk::PinataApi::new(public_key, secret_key).unwrap();

    let mut json_data = HashMap::new();
    json_data.insert("image", b64);

    let res = pinata.pin_json(PinByJson::new(json_data)).await.unwrap();

    reqwest::get(format!("{}/execute/{}", execution_service, res.ipfs_hash)).await?;

    println!("Task created");

    Ok(())
}

fn decrypt_image(
    keys_path: &PathBuf,
    encrypted_image: &PathBuf,
    image_path: &PathBuf,
) -> Result<()> {
    let keys = cryptography::Keys::load(keys_path.to_string_lossy().to_string())?;

    let mut file = File::open(encrypted_image)?;
    let mut enc_b64 = Vec::<u8>::new();
    file.read_to_end(&mut enc_b64)?;

    let enc = cryptography::decode_enc_image(&String::from_utf8(enc_b64)?)?;
    let dec = keys.dec_array(enc);

    images::save_image(&dec, image_path.to_string_lossy().to_string())?;

    Ok(())
}

async fn fetch_image(
    keys_path: &PathBuf,
    ipfs_hash: &String,
    image_path: &PathBuf
) -> Result<()> {
    let keys = cryptography::Keys::load(keys_path.to_string_lossy().to_string())?;
    
    let data = reqwest::get(format!("https://ipfs.io/ipfs/{}", ipfs_hash)).await?;
    let value: Value = data.json().await?;
    let image_val = value.get("image").unwrap();
    let image = image_val.as_str().unwrap();

    let enc = cryptography::decode_enc_image(&image.to_string())?;
    
    let dec = keys.dec_array(enc);
    
    images::save_image(&dec, image_path.to_string_lossy().to_string())?;

    Ok(())
}