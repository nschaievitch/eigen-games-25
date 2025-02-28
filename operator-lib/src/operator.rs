use eyre::Result;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;
use tfhe::{prelude::*, ClearArray, CpuFheUint8Array, CpuFheUint8Slice, FheInt16, FheInt8, FheUint, FheUint8Id, ServerKey};
use tfhe::{ConfigBuilder, FheUint8, FheUint32, generate_keys, set_server_key};
use base64::alphabet::Alphabet;
use base64::{Engine as _, engine::general_purpose};
use bincode;
use itertools::Itertools;
use regex::Regex;

fn convolve_enc(image: Vec<FheUint8>, kernel: Vec<i16>) -> Vec<FheUint8> {
    // assume image is 32*32 and kernel is 3*3

    let mut result: Vec<FheInt16> = Vec::new();

    for i in 0..30 {
        println!("Row {i}...");
        for j in 0..30 {
            let idx = i * 32 + j; // top left corner
            let mut res: FheInt16 = FheInt16::cast_from(image[idx].clone());
            res *= kernel[0].clone();
            res += kernel[1] * FheInt16::cast_from(image[idx + 1].clone());
            res += kernel[2] * FheInt16::cast_from(image[idx + 2].clone());
            res += kernel[3] * FheInt16::cast_from(image[idx + 32].clone());
            res += kernel[4] * FheInt16::cast_from(image[idx + 33].clone());
            res += kernel[5] * FheInt16::cast_from(image[idx + 34].clone());
            res += kernel[6] * FheInt16::cast_from(image[idx + 64].clone());
            res += kernel[7] * FheInt16::cast_from(image[idx + 65].clone());
            res += kernel[8] * FheInt16::cast_from(image[idx + 66].clone());

            result.push(res.min(255).max(0));
        }

        result.push(image[i * 32 + 30].clone().cast_into());
        result.push(image[i * 32 + 31].clone().cast_into())
    }

    // add two last rows
    for i in 30*32..32*32 {
        result.push(image[i].clone().cast_into());
    }

    result.into_iter().map(|v| v.cast_into()).collect()
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

pub fn process_encrypted_image(encoded_image: &String, server_key: ServerKey) -> Result<String> {
    set_server_key(server_key);

    let sharpen: Vec<i16> = vec![0, -1, 0, -1, 5, -1, 0, -1, 0];
    // decode
    let enc = decode_enc_image(encoded_image)?;
    
    // process image
    let processed = convolve_enc(enc, sharpen);

    // re-encode
    let res = encode_enc_image(processed)?;
    
    Ok(res)

}
