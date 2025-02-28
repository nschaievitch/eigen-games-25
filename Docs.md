# Documentation

## General workflow

The general workflow of the service is as follows:

1. The client, having generated the keys, calls the `process-image` subcommand. This does the following operations:
   - Loads the image, re-scales it to the required size, and converts to a greyscale flat u8 array.
   - Encrypts the image with the provided keys
   - Submits the image to IPFS. This is encoded to a base64 string, and then compressed using run-length encoding. In the future, adapting to binary formats could allow for smaller sizes.
   - Sends the `execute` request, along with the IPFS hash of the image. As we currently use dummy keys (due to the real encryption requiring powerful hardware), we only send over the image. The full product would also send the serialized server keys.
2. The ExecutionService gets the request and does the following:
   - Fetches the image from IPFS
   - Decodes the image to an array of encrypted u8s
   - Performs the convolution with the sharpen kernel
   - Submits the original and processed images to IPFS, encoded with the same format as above
   - Submits the proof of task (with both IPFS content IDs)
3. The ValidationService gets the proof of task, fetches the images, and performs the same FHE computation to compare against the provided processed image.S

## Client

The client provides a Rust CLI for interacting with the AVS.

### Usage
  
- `generate-keys`: Generate a new set of encryption keys
- `process-image`: Process and encrypt an image
- `decrypt-image`: Decrypt an encrypted image
- `fetch-image`: Fetch image from IPFS proof of task
- `help`: Print this message or the help of the given subcommand(s)

The CLI is built with clap, so information on each subcommand can be found with `./client help <subcommand>`

### Building

With Rust installed, it is enough to run `cargo build` and copy over the resulting binary.

## Services

###  Running

Follow the Othentic quickstart guide [here](https://docs.othentic.xyz/main).

Ensure that in the `.env` file as per the quick start guide all of the private keys are filled in, as well as funded properly. 

After following the guide run these commands in order in the terminal:

```bash
docker-compose build
docker-compose up
```

## Operator-lib

This code contains the logic behind the FHE image processing, to be run by the Execution and Validation services. `operator.rs` has the functions for encoding/decoding, and running the convolution on the encrypted image.

For more information on the convolution algorithm, check the `sharpen` kernel [here](https://en.wikipedia.org/wiki/Kernel_(image_processing)).

The actual FHE logic is handled by the [TFHE-rs library](https://docs.zama.ai/tfhe-rs), from Zama AI. 

### Building

With Rust installed, it is enough to run `cargo build` and copy over the resulting binary to the `Execution_Service` and `Validation_Service`

## Pinata-sdk

This is just a quick modification of the library to use the Othentic IPFS SDK.
