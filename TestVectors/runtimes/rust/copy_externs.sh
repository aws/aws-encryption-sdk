#!/bin/bash -eu

cd $( dirname ${BASH_SOURCE[0]} )

SRC=../../../mpl/AwsCryptographicMaterialProviders/runtimes/rust/src/

cp $SRC/aes_gcm.rs src
cp $SRC/aes_kdf_ctr.rs src
cp $SRC/concurrent_call.rs src
cp $SRC/dafny_libraries.rs src
cp $SRC/ddb.rs src
cp $SRC/digest.rs src
cp $SRC/ecdh.rs src
cp $SRC/ecdsa.rs src
cp $SRC/escape.rs src
cp $SRC/hmac.rs src
cp $SRC/kms.rs src
cp $SRC/local_cmc.rs src
cp $SRC/oslang.rs src
cp $SRC/random.rs src
cp $SRC/rsa.rs src
cp $SRC/sets.rs src
cp $SRC/software_externs.rs src
cp $SRC/storm_tracker.rs src
cp $SRC/time.rs src
cp $SRC/uuid.rs src
