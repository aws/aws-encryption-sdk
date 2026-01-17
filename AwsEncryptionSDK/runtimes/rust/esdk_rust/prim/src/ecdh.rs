#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[allow(missing_docs)]
#[non_exhaustive]
pub enum EcdhCurveSpec {
    EccNistP256,
    EccNistP384,
    #[default]
    EccNistP521,
}
