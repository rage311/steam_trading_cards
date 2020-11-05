use openssl::{bn, rsa};
use std::error::Error;
use std::io;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct RSAParams {
    pub modulo: String,
    pub exponent: String,
    pub timestamp: String,
}

pub fn rsa_encrypt(rsa: &RSAParams, plaintext: String) -> Result<String, Box<dyn Error>> {
    let bn_mod = bn::BigNum::from_hex_str(rsa.modulo.as_str())?;
    let bn_exp = bn::BigNum::from_hex_str(rsa.exponent.as_str())?;

    let key = rsa::Rsa::from_public_components(bn_mod, bn_exp)?;

    let mut pass_crypt = vec![0; key.size() as usize];
    let _encrypted_len =
        key.public_encrypt(plaintext.as_bytes(), &mut pass_crypt, rsa::Padding::PKCS1);

    Ok(base64::encode(pass_crypt))
}

pub fn timestamp_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

pub fn list_price(lowest_price: u64, adjustment: i64) -> u64 {
    let gross_price_cents = lowest_price as i64 + adjustment;
    let initial_net_price = (gross_price_cents as f64 * 0.87) as i64;
    let diff = gross_price_cents - initial_net_price;
    let net_price_cents = match diff <= 2 {
        true => match gross_price_cents - 2 > 0 {
            true => gross_price_cents - 2,
            false => 2,
        },
        false => initial_net_price + 1,
    };

    dbg!(gross_price_cents);
    dbg!(initial_net_price);
    dbg!(diff);
    dbg!(net_price_cents);

    net_price_cents as u64
}

pub fn two_factor_prompt() -> Option<String> {
    let mut buf = String::new();
    print!("Steam two factor code (not case sensitive): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buf).unwrap();
    Some(buf)
}
