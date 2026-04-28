// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![allow(dead_code)]
use crate::utils::*;
use crate::*;

fn make_canon(table_name: &str, data: &CryptoItem) -> CanonCryptoItem {
    CanonCryptoItem {
        key: paths::canon_path(table_name, &data.key),
        orig_key: data.key.clone(),
        data: data.data.clone(),
        action: data.action,
    }
}
fn make_canon_auth(table_name: &str, data: &AuthItem) -> CanonAuthItem {
    CanonAuthItem {
        key: paths::canon_path(table_name, &data.key),
        orig_key: data.key.clone(),
        data: data.data.clone(),
        action: data.action,
    }
}

fn make_crypto_item(x: &CanonAuthItem, action: CryptoAction) -> CanonCryptoItem {
    CanonCryptoItem {
        key: x.key.clone(),
        orig_key: x.orig_key.clone(),
        data: x.data.clone(),
        action,
    }
}

const fn legend_to_action(v: header::LegendByte) -> CryptoAction {
    if v == header::ENCRYPT_AND_SIGN_LEGEND {
        CryptoAction::EncryptAndSign
    } else if v == header::SIGN_AND_INCLUDE_IN_ENCRYPTION_CONTEXT_LEGEND {
        CryptoAction::SignAndIncludeInEncryptionContext
    } else {
        CryptoAction::SignOnly
    }
}

// TODO - take fields by value
fn resolve_legend(
    fields: &[CanonAuthItem],
    legend: &[header::LegendByte],
) -> Result<Vec<CanonCryptoItem>, Error> {
    let mut ret = Vec::new();
    let mut legend_pos = 0usize;
    for field in fields {
        if field.action == AuthenticateAction::DoNotSign {
            ret.push(make_crypto_item(field, CryptoAction::DoNothing));
        } else {
            need(
                legend_pos < legend.len(),
                "Schema changed : something that was unsigned is now signed.",
            )?;
            ret.push(make_crypto_item(
                field,
                legend_to_action(legend[legend_pos]),
            ));
            legend_pos += 1;
        }
    }
    need(
        legend_pos == legend.len(),
        "Schema changed : something that was signed is now unsigned.",
    )?;
    Ok(ret)
}

fn auth_to_canon_auth(table_name: &str, data: &[AuthItem]) -> Vec<CanonAuthItem> {
    let mut canon_list: Vec<CanonAuthItem> = Vec::with_capacity(data.len());
    for item in data {
        canon_list.push(make_canon_auth(table_name, item));
    }
    canon_list
}

fn crypto_to_canon_crypto(table_name: &str, data: &[CryptoItem]) -> Vec<CanonCryptoItem> {
    let mut canon_list: Vec<CanonCryptoItem> = Vec::with_capacity(data.len());
    for item in data {
        canon_list.push(make_canon(table_name, item));
    }
    canon_list
}

fn auth_sort(mut canon_list: Vec<CanonAuthItem>) -> Vec<CanonAuthItem> {
    canon_list.sort_unstable_by(|a, b| a.key.cmp(&b.key));
    canon_list
}

fn crypto_sort(mut canon_list: Vec<CanonCryptoItem>) -> Vec<CanonCryptoItem> {
    canon_list.sort_unstable_by(|a, b| a.key.cmp(&b.key));
    canon_list
}

fn for_encrypt(table_name: &str, data: &[CryptoItem]) -> Vec<CanonCryptoItem> {
    let canon_list = crypto_to_canon_crypto(table_name, data);
    crypto_sort(canon_list)
}

pub(crate) fn for_decrypt(
    table_name: &str,
    data: &[AuthItem],
    legend: &[header::LegendByte],
) -> Result<Vec<CanonCryptoItem>, Error> {
    let canon_list = auth_to_canon_auth(table_name, data);
    let canon_list = auth_sort(canon_list);
    resolve_legend(&canon_list, legend)
}

pub(crate) fn un_canon(input: &[CanonCryptoItem]) -> Vec<CryptoItem> {
    let mut result = Vec::with_capacity(input.len());
    for item in input {
        result.push(CryptoItem {
            key: item.orig_key.clone(),
            data: item.data.clone(),
            action: item.action,
        });
    }
    result
}

pub(crate) fn add_headers(
    mut input: Vec<CryptoItem>,
    header_data: StructuredDataTerminal,
    footer_data: StructuredDataTerminal,
) -> Vec<CryptoItem> {
    let head_item = CryptoItem {
        key: (*HEADER_PATH).clone(),
        data: header_data,
        action: CryptoAction::DoNothing,
    };
    let foot_item = CryptoItem {
        key: (*FOOTER_PATH).clone(),
        data: footer_data,
        action: CryptoAction::DoNothing,
    };
    input.push(head_item);
    input.push(foot_item);
    input
}

pub(crate) fn remove_headers(mut input: Vec<CryptoItem>) -> Vec<CryptoItem> {
    input.retain(|x| x.key != *HEADER_PATH && x.key != *FOOTER_PATH);
    input
}
