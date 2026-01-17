use crate::DigestAlg;
use crate::Error;
use crate::serr;

#[derive(Debug, Clone)]
pub struct Prk {
    prk: aws_lc_rs::hkdf::Prk,
}

const fn get_alg(alg: DigestAlg) -> aws_lc_rs::hkdf::Algorithm {
    match alg {
        DigestAlg::Sha256 => aws_lc_rs::hkdf::HKDF_SHA256,
        DigestAlg::Sha384 => aws_lc_rs::hkdf::HKDF_SHA384,
        DigestAlg::Sha512 => aws_lc_rs::hkdf::HKDF_SHA512,
    }
}

fn get_len(len: usize) -> Result<&'static aws_lc_rs::aead::Algorithm, Error> {
    match len {
        16 => Ok(&aws_lc_rs::aead::AES_128_GCM),
        24 => Ok(&aws_lc_rs::aead::AES_192_GCM),
        32 => Ok(&aws_lc_rs::aead::AES_256_GCM),
        _ => Err(serr(format!("Invalid key length {len} passed to HKDF"))),
    }
}

pub fn hkdf(
    alg: DigestAlg,
    salt: &[u8],
    ikm: &[u8],
    info: &[&[u8]],
    okm: &mut [u8],
) -> Result<(), Error> {
    let salt = aws_lc_rs::hkdf::Salt::new(get_alg(alg), salt);
    let prk = salt.extract(ikm);
    let inner_okm = prk
        .expand(info, get_len(okm.len())?)
        .map_err(|e| serr(format!("{e:?}")))?;
    inner_okm.fill(okm).unwrap();
    Ok(())
}

#[must_use]
pub fn hkdf_extract(alg: DigestAlg, salt: &[u8], ikm: &[u8]) -> Prk {
    let salt = aws_lc_rs::hkdf::Salt::new(get_alg(alg), salt);
    let prk = salt.extract(ikm);
    Prk { prk }
}

pub fn hkdf_expand(prk: &Prk, info: &[&[u8]], okm: &mut [u8]) -> Result<(), Error> {
    let inner_okm = prk
        .prk
        .expand(info, get_len(okm.len())?)
        .map_err(|e| serr(format!("{e:?}")))?;
    inner_okm.fill(okm).unwrap();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "track")]
    #[test]
    fn test_hkdf_raw() {
        let tr = crate::memtracker::ResourceTracker::new();
        let mut okm = [0u8; 32];
        let ikm = [0u8; 32];
        let salt = aws_lc_rs::hkdf::Salt::new(aws_lc_rs::hkdf::HKDF_SHA256, &[1u8, 2, 3, 4, 5]);
        tr.report("made salt");
        let prk = salt.extract(&ikm);
        tr.report("after extract");
        let inner_okm = prk
            .expand(&[&[1u8, 2, 3, 4, 5]], &aws_lc_rs::aead::AES_256_GCM)
            .unwrap();
        tr.report("after expand");
        inner_okm.fill(&mut okm).unwrap();
        tr.report("after fill");
    }

    #[test]
    fn test_hkdf() -> Result<(), Error> {
        let mut okm = [0u8; 32];
        let ikm = [0u8; 32];
        hkdf(
            DigestAlg::Sha512,
            &[1u8, 2, 3, 4, 5],
            &ikm,
            &[&[1u8, 2, 3, 4, 5]],
            &mut okm,
        )?;
        // println!("{:?}", okm);
        Ok(())
    }
    // fn hkdf3(salt: &[u8], ikm: &[u8], info: &[&[u8]], okm: &mut [u8]) {
    //     let salt = aws_lc_rs::hkdf::Salt::new(aws_lc_rs::hkdf::HKDF_SHA256, salt);
    //     let prk = salt.extract(&ikm);
    //     let inner_okm = prk
    //         .expand(info, &aws_lc_rs::aead::AES_256_GCM)
    //         .unwrap();
    //     inner_okm.fill(okm).unwrap();
    // }
}
