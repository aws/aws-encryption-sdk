#[derive(Debug, PartialEq, Copy, Clone, Hash, Default)]
#[allow(missing_docs)]
#[non_exhaustive]
pub enum EcdhCurveSpec {
    EccNistP256,
    EccNistP384,
    #[default]
    EccNistP521,
}
