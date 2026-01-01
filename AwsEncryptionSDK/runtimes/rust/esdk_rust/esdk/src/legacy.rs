use aws_mpl_legacy::types::AlgorithmSuiteId as Old;
use aws_mpl_legacy::types::DbeAlgorithmSuiteId as OldDbe;
use aws_mpl_legacy::types::EsdkAlgorithmSuiteId as OldEsdk;
use aws_mpl_rs::suites::AlgorithmSuiteId;
use aws_mpl_rs::suites::DbeAlgorithmSuiteId;
use aws_mpl_rs::suites::EsdkAlgorithmSuiteId;

use crate::*;

pub(crate) const fn from_legacy_esdk_suite_id(legacy: OldEsdk) -> EsdkAlgorithmSuiteId {
    match legacy {
        OldEsdk::AlgAes128GcmIv12Tag16NoKdf => EsdkAlgorithmSuiteId::AlgAes128GcmIv12Tag16NoKdf,
        OldEsdk::AlgAes192GcmIv12Tag16NoKdf => EsdkAlgorithmSuiteId::AlgAes192GcmIv12Tag16NoKdf,
        OldEsdk::AlgAes256GcmIv12Tag16NoKdf => EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16NoKdf,
        OldEsdk::AlgAes128GcmIv12Tag16HkdfSha256 => {
            EsdkAlgorithmSuiteId::AlgAes128GcmIv12Tag16HkdfSha256
        }
        OldEsdk::AlgAes192GcmIv12Tag16HkdfSha256 => {
            EsdkAlgorithmSuiteId::AlgAes192GcmIv12Tag16HkdfSha256
        }
        OldEsdk::AlgAes256GcmIv12Tag16HkdfSha256 => {
            EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256
        }
        OldEsdk::AlgAes128GcmIv12Tag16HkdfSha256EcdsaP256 => {
            EsdkAlgorithmSuiteId::AlgAes128GcmIv12Tag16HkdfSha256EcdsaP256
        }
        OldEsdk::AlgAes192GcmIv12Tag16HkdfSha384EcdsaP384 => {
            EsdkAlgorithmSuiteId::AlgAes192GcmIv12Tag16HkdfSha384EcdsaP384
        }
        OldEsdk::AlgAes256GcmIv12Tag16HkdfSha384EcdsaP384 => {
            EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha384EcdsaP384
        }
        OldEsdk::AlgAes256GcmHkdfSha512CommitKey => {
            EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey
        }
        OldEsdk::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384 => {
            EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384
        }
    }
}

pub(crate) const fn from_legacy_dbe_suite_id(legacy: OldDbe) -> DbeAlgorithmSuiteId {
    match legacy {
        OldDbe::AlgAes256GcmHkdfSha512CommitKeySymsigHmacSha384 => {
            DbeAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeySymsigHmacSha384
        }
        OldDbe::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384SymsigHmacSha384 => {
            DbeAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384SymsigHmacSha384
        }
    }
}

pub(crate) fn from_legacy_suite_id(legacy: &Old) -> Result<AlgorithmSuiteId, Error> {
    match legacy {
        Old::Esdk(x) => Ok(AlgorithmSuiteId::Esdk(from_legacy_esdk_suite_id(*x))),
        Old::Dbe(x) => Ok(AlgorithmSuiteId::Dbe(from_legacy_dbe_suite_id(*x))),
        _ => Err(val_err("Unrecognized legacy AlgorithmSuiteId")),
    }
}
