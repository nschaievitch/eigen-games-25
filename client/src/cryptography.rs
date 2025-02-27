use base64::alphabet::Alphabet;
use base64::{Engine as _, engine::general_purpose};
use bincode;
use eyre::Result;
use itertools::Itertools;
use tfhe::{prelude::*, set_server_key, ClientKey, FheUint8, ServerKey};
use tfhe::{ConfigBuilder, generate_keys};
use std::fs::File;
use std::io::{Read, Write};
use regex::Regex;

pub struct Keys {
    pub client_key: ClientKey,
    pub server_key: ServerKey
}

impl Keys {
    pub fn new() -> Keys {
        let config = ConfigBuilder::default().build();
        let (csk, srk) = generate_keys(config);

        set_server_key(srk.clone());

        Keys {
            client_key: csk,
            server_key: srk,
        }
    }

    pub fn enc_array(&self, arr: Vec<u8>) -> Vec<FheUint8> {
        let enc: Vec<FheUint8> = arr
            .into_iter()
            .map(|v| FheUint8::try_encrypt_trivial(v).unwrap())
            .collect::<Vec<FheUint8>>();

        enc
    }

    pub fn dec_array(&self, arr: Vec<FheUint8>) -> Vec<u8> {
        let dec: Vec<u8> = arr
            .into_iter()
            .map(|v| v.try_decrypt_trivial::<u8>().unwrap())
            .collect::<Vec<u8>>();

        dec
    }

    pub fn save(&self, path: String) -> Result<()> {
        let bin_key = bincode::serialize(&self.client_key)?;

        let mut file = File::create(path)?;
        file.write_all(&bin_key)?;

        Ok(())
    }

    pub fn load(path: String) -> Result<Keys> {
        let mut file = File::open(path)?;
        let mut bin_key = Vec::<u8>::new();
        file.read_to_end(&mut bin_key)?;

        let client_key: ClientKey = bincode::deserialize(&bin_key)?;
        let server_key = client_key.generate_server_key();

        set_server_key(server_key.clone());

        Ok(Keys {
            client_key: client_key,
            server_key: server_key
        })
    }
}

fn run_length_encode(input: &str) -> String {
    input
        .chars()
        .group_by(|&c| c)
        .into_iter()
        .map(|(key, group)| {
            let count = group.count();
            format!("{}{}", key, count)
        })
        .collect()
}

fn run_length_decode(encoded: &str) -> String {
    let re = Regex::new(r"([A-Za-z])(\d+)").unwrap();
    re.captures_iter(encoded)
        .flat_map(|cap| {
            let ch = &cap[1];
            let count: usize = cap[2].parse().unwrap();
            ch.repeat(count).chars().collect::<Vec<_>>()
        })
        .collect()
}

pub fn encode_enc_image(enc: Vec<FheUint8>) -> Result<String> {
    let alph: Alphabet = Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!@#$%^&*()-_").unwrap();
    let engine = base64::engine::GeneralPurpose::new(
        &alph,
        base64::engine::general_purpose::NO_PAD
    );

    let bin = bincode::serialize(&enc)?;
    let b64 = engine.encode(bin);
    let rle_b64 = run_length_encode(&b64);

    Ok(rle_b64)
}

pub fn decode_enc_image(rle_b64: &String) -> Result<Vec<FheUint8>> {
    let alph: Alphabet = Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!@#$%^&*()-_").unwrap();
    let engine = base64::engine::GeneralPurpose::new(
        &alph,
        base64::engine::general_purpose::NO_PAD
    );

    let b64 = run_length_decode(&rle_b64);
    let bin = engine.decode(b64)?;
    let enc: Vec<FheUint8> = bincode::deserialize(&bin)?;

    Ok(enc)
}