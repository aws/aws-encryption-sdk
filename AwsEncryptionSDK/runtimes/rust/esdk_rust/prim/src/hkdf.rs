
use crate::Error;
use crate::serr;
use crate::DigestAlg;

#[derive(Debug, Clone)]
pub struct Prk {
    prk : aws_lc_rs::hkdf::Prk
}

fn get_alg(alg : DigestAlg) -> aws_lc_rs::hkdf::Algorithm
{
    match alg {
        DigestAlg::Sha256 => aws_lc_rs::hkdf::HKDF_SHA256,
        DigestAlg::Sha384 => aws_lc_rs::hkdf::HKDF_SHA384,
        DigestAlg::Sha512 => aws_lc_rs::hkdf::HKDF_SHA512,
    }
}

fn get_len(len : usize) -> Result<&'static aws_lc_rs::aead::Algorithm, Error>
{
    match len {
        16 => Ok(&aws_lc_rs::aead::AES_128_GCM),
        24 => Ok(&aws_lc_rs::aead::AES_192_GCM),
        32 => Ok(&aws_lc_rs::aead::AES_256_GCM),
        _ => Err(serr(format!("Invalid key length {len} passed to HKDF")))
    }
}
pub fn hkdf(alg: DigestAlg, salt: &[u8], ikm: &[u8], info: &[u8], okm: &mut [u8]) -> Result<(), Error> {
    let salt = aws_lc_rs::hkdf::Salt::new(get_alg(alg), salt);
    let prk = salt.extract(ikm);
    let info_list = [info];
    let inner_okm = prk.expand(&info_list, get_len(okm.len())?).map_err(|e| serr(format!("{e:?}")))?;
    inner_okm.fill(okm).unwrap();
    Ok(())
}

#[must_use]
pub fn hkdf_extract(alg: DigestAlg, salt: &[u8], ikm: &[u8]) -> Prk {
    let salt = aws_lc_rs::hkdf::Salt::new(get_alg(alg), salt);
    let prk = salt.extract(ikm);
    Prk{prk}
}

pub fn hkdf_expand(prk: &Prk, info: &[u8], okm: &mut [u8]) -> Result<(), Error> {
    let info_list = [info];
    let inner_okm = prk.prk.expand(&info_list, get_len(okm.len())?).map_err(|e| serr(format!("{e:?}")))?;
    inner_okm.fill(okm).unwrap();
    Ok(())
}
