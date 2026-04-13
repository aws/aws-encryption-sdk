pub(crate) mod body;
pub(crate) mod encrypted_data_keys;
pub(crate) mod encryption_context;
pub(crate) mod footer;
pub(crate) mod header;
pub(crate) mod header_auth;
pub(crate) mod header_types;
pub(crate) mod serializable_types;
pub(crate) mod serialize_functions;
pub(crate) mod shared_header_functions;
pub(crate) mod v1_header_body;
pub(crate) mod v2_header_body;
use aws_mpl_legacy::primitives::DigestContext;

pub(crate) use super::error::Error;
pub(crate) use super::error::ser_err;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub(crate) struct NoopWriter;
impl std::io::Write for NoopWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct DigestWriter {
    pub(crate) context: Option<DigestContext>,
}
impl DigestWriter {
    #[expect(dead_code)]
    pub(crate) fn new(alg: aws_mpl_legacy::primitives::DigestAlg) -> Result<Self, Error> {
        let context = Some(DigestContext::new(alg)?);
        Ok(Self { context })
    }
    #[expect(dead_code)]
    pub(crate) fn from_ecdsa(
        alg: aws_mpl_legacy::primitives::EcdsaSignatureAlgorithm,
    ) -> Result<Self, Error> {
        let context = Some(DigestContext::new_from_ecdsa(alg)?);
        Ok(Self { context })
    }
    pub(crate) fn from_old_ecdsa(
        alg: aws_mpl_legacy::suites::SignatureAlgorithm,
    ) -> Result<Self, Error> {
        match alg {
            aws_mpl_legacy::suites::SignatureAlgorithm::Ecdsa(x) => {
                let context = Some(DigestContext::new_from_ecdsa(x)?);
                Ok(Self { context })
            }
            _ => Ok(Self { context: None }),
        }
    }
}

impl std::io::Write for DigestWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if let Some(ref mut context) = self.context {
            context.update(buf);
        }
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
