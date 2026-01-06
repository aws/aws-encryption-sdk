use crate::DigestAlg;
use crate::Error;
use crate::serr;

use aws_lc_rs::rand::SystemRandom;
use aws_lc_rs::signature::EcdsaKeyPair;
use aws_lc_rs::signature::EcdsaSigningAlgorithm;
use aws_lc_rs::signature::EcdsaVerificationAlgorithm;
use aws_lc_rs::signature::UnparsedPublicKey;

#[derive(Clone)]
pub struct DigestContext {
    context: aws_lc_rs::digest::Context,
}

impl std::fmt::Debug for DigestContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DigestContext {:?}", self.context.algorithm())
    }
}

impl DigestContext {
    pub fn new(alg: DigestAlg) -> Result<Self, Error> {
        let alg = get_digest_alg(alg);
        Ok(Self {
            context: aws_lc_rs::digest::Context::new(alg),
        })
    }
    pub fn new_from_ecdsa(alg: EcdsaSignatureAlgorithm) -> Result<Self, Error> {
        let alg = get_digest_alg_from_ecdsa(alg);
        Ok(Self {
            context: aws_lc_rs::digest::Context::new(alg),
        })
    }
    pub fn update(&mut self, data: &[u8]) {
        self.context.update(data);
    }
    #[must_use]
    pub fn finish(self) -> Vec<u8> {
        self.context.finish().as_ref().to_vec()
    }
    fn digest(self) -> aws_lc_rs::digest::Digest {
        self.context.finish()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Hash)]
#[non_exhaustive]
pub enum EcdsaSignatureAlgorithm {
    EcdsaP384,
    #[default]
    EcdsaP256,
}

const fn get_digest_alg(x: DigestAlg) -> &'static aws_lc_rs::digest::Algorithm {
    match x {
        DigestAlg::Sha256 => &aws_lc_rs::digest::SHA256,
        DigestAlg::Sha384 => &aws_lc_rs::digest::SHA384,
        DigestAlg::Sha512 => &aws_lc_rs::digest::SHA512,
    }
}

const fn get_digest_alg_from_ecdsa(
    x: EcdsaSignatureAlgorithm,
) -> &'static aws_lc_rs::digest::Algorithm {
    match x {
        EcdsaSignatureAlgorithm::EcdsaP256 => &aws_lc_rs::digest::SHA256,
        EcdsaSignatureAlgorithm::EcdsaP384 => &aws_lc_rs::digest::SHA384,
    }
}

const fn get_sign_alg(x: EcdsaSignatureAlgorithm) -> &'static EcdsaSigningAlgorithm {
    match x {
        EcdsaSignatureAlgorithm::EcdsaP256 => &aws_lc_rs::signature::ECDSA_P256_SHA256_ASN1_SIGNING,
        EcdsaSignatureAlgorithm::EcdsaP384 => &aws_lc_rs::signature::ECDSA_P384_SHA384_ASN1_SIGNING,
    }
}

const fn get_ver_alg(x: EcdsaSignatureAlgorithm) -> &'static EcdsaVerificationAlgorithm {
    match x {
        EcdsaSignatureAlgorithm::EcdsaP256 => &aws_lc_rs::signature::ECDSA_P256_SHA256_ASN1,
        EcdsaSignatureAlgorithm::EcdsaP384 => &aws_lc_rs::signature::ECDSA_P384_SHA384_ASN1,
    }
}

pub fn ecdsa_sign(alg: EcdsaSignatureAlgorithm, key: &[u8], msg: &[u8]) -> Result<Vec<u8>, Error> {
    // This loop can in theory run forever, but the chances of that are negligible.
    // We may want to consider failing, after some number of loops, if we can do so in a way consistent with other ESDKs.
    let alg = get_sign_alg(alg);
    let private_key: EcdsaKeyPair =
        EcdsaKeyPair::from_private_key_der(alg, key).map_err(|e| serr(format!("{e:?}")))?;
    let rng = SystemRandom::new();
    loop {
        let result: aws_lc_rs::signature::Signature = private_key
            .sign(&rng, msg)
            .map_err(|e| serr(format!("{e:?}")))?;

        if (alg == &aws_lc_rs::signature::ECDSA_P384_SHA384_ASN1_SIGNING
            && result.as_ref().len() == 103)
            || (alg == &aws_lc_rs::signature::ECDSA_P256_SHA256_ASN1_SIGNING
                && result.as_ref().len() == 71)
        {
            return Ok(result.as_ref().into());
        }
    }
}

pub fn ecdsa_sign_digest(
    alg: EcdsaSignatureAlgorithm,
    key: &[u8],
    context: DigestContext,
) -> Result<Vec<u8>, Error> {
    let alg = get_sign_alg(alg);
    let digest = context.digest();
    let private_key: EcdsaKeyPair =
        EcdsaKeyPair::from_private_key_der(alg, key).map_err(|e| serr(format!("{e:?}")))?;

    // This loop can in theory run forever, but the chances of that are negligible.
    // We may want to consider failing, after some number of loops, if we can do so in a way consistent with other ESDKs.
    loop {
        let result: aws_lc_rs::signature::Signature = private_key
            .sign_digest(&digest)
            .map_err(|e| serr(format!("{e:?}")))?;

        if (alg == &aws_lc_rs::signature::ECDSA_P384_SHA384_ASN1_SIGNING
            && result.as_ref().len() == 103)
            || (alg == &aws_lc_rs::signature::ECDSA_P256_SHA256_ASN1_SIGNING
                && result.as_ref().len() == 71)
        {
            return Ok(result.as_ref().into());
        }
    }
}

pub fn ecdsa_verify(
    alg: EcdsaSignatureAlgorithm,
    key: &[u8],
    msg: &[u8],
    sig: &[u8],
) -> Result<bool, Error> {
    let public_key = UnparsedPublicKey::new(get_ver_alg(alg), key);
    match public_key.verify(msg, sig) {
        Ok(()) => Ok(true),
        Err(_) => Ok(false),
    }
}

pub fn ecdsa_verify_context(
    alg: EcdsaSignatureAlgorithm,
    key: &[u8],
    context: DigestContext,
    sig: &[u8],
) -> Result<bool, Error> {
    let public_key = UnparsedPublicKey::new(get_ver_alg(alg), key);
    let digest = context.digest();
    match public_key.verify_digest(&digest, sig) {
        Ok(()) => Ok(true),
        Err(_) => Ok(false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aws_lc_rs::encoding::AsDer;
    use aws_lc_rs::signature::KeyPair;

    #[test]
    fn test_ecdsa() -> Result<(), Error> {
        let alg = EcdsaSignatureAlgorithm::EcdsaP256;
        let pair =
            EcdsaKeyPair::generate(&aws_lc_rs::signature::ECDSA_P256_SHA256_ASN1_SIGNING).unwrap();
        let public_key: Vec<u8> = pair.public_key().as_ref().into();
        let private_key: Vec<u8> = pair.private_key().as_der().unwrap().as_ref().into();

        let data = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let sign1 = ecdsa_sign(alg, &private_key, &data)?;

        let mut context = DigestContext::new_from_ecdsa(alg)?;
        context.update(&data);
        let sign2: Vec<u8> = ecdsa_sign_digest(alg, &private_key, context)?;

        assert!(ecdsa_verify(alg, &public_key, &data, &sign1)?);
        assert!(ecdsa_verify(alg, &public_key, &data, &sign2)?);

        let mut context = DigestContext::new_from_ecdsa(alg)?;
        context.update(&data);
        assert!(ecdsa_verify_context(alg, &public_key, context, &sign1)?);
        let mut context = DigestContext::new_from_ecdsa(alg)?;
        context.update(&data);
        assert!(ecdsa_verify_context(alg, &public_key, context, &sign2)?);

        Ok(())
    }
}
