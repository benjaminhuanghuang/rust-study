use base64::decode;

use std::str;

use std::error::Error;

fn decoded() -> Result<(), Box<dyn Error>> {
  let b64 = "V2VsY29tZSB0byBMaW51eGhpbnQ=";

  let decoded = &decode(b64).unwrap()[..];

  println!("String: {:?}", str::from_utf8(decoded));

  Ok(())
}

fn encoded() -> Result<(), Box<dyn Error>> {
  let string = b"Welcome to Linuxhint";

  let encoded = encode(string);

  println!("Base64: {}", encoded);

  Ok(())
}
