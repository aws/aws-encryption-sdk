// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

// The only entry point currently used is simple_canon
// which converts a top level terminal to its canonical form

#![allow(dead_code)]
use crate::serialize::*;
use crate::utils::*;
use crate::*;

pub(crate) enum Selector {
    List(usize),
    Map(String),
}

pub(crate) fn string_to_uni_path(attr: String) -> Vec<PathSegment> {
    vec![PathSegment::new(attr)]
}

pub(crate) fn uni_path_to_string(x: &[PathSegment]) -> String {
    match &x[0] {
        PathSegment::Member(StructureSegment { key }) => key.clone(),
    }
}

pub(crate) fn canon_path(table: &str, path: &[PathSegment]) -> CanonicalPath
//= specification/structured-encryption/header.md#canonical-path
    //= type=implication
    //# The canonical path MUST start with the UTF8 encoded table name.

    //= specification/structured-encryption/header.md#canonical-path
    //= type=implication
    //# This MUST be followed by the depth of the Terminal within Structured Data.

    //= specification/structured-encryption/header.md#canonical-path
    //= type=implication
    //# This MUST be followed by the encoding for each Structured Data in the path, including the Terminal itself.
{
    let mut ret: CanonicalPath = Vec::new();
    ret.extend_from_slice(table.as_bytes());
    write_u64(&mut ret, path.len() as u64);
    make_canonical_path(&mut ret, path);
    ret
}

pub(crate) fn term_loc_map(attr: &str) -> Vec<PathSegment> {
    vec![PathSegment::new(attr.to_string())]
}

pub(crate) fn simple_canon(table: &str, attr: &str) -> CanonicalPath {
    canon_path(table, &term_loc_map(attr))
}

const ARRAY_TAG: u8 = b'#';
const MAP_TAG: u8 = b'$';

// get the Canonical Path fragment for this Selector
fn canonical_part(data: &mut Vec<u8>, s: &PathSegment)
//= specification/structured-encryption/header.md#canonical-path
//= type=implication
//# For Structured Data in Structured Data Maps, this MUST be a 0x24 byte ($ in UTF-8),
//# followed by the length of the key, followed by the key as a UTF8 string.
{
    data.push(MAP_TAG);
    write_u32(data, s.key().len() as u32);
    data.extend_from_slice(s.key().as_bytes());
}

// get the Canonical Path for these Selectors
fn make_canonical_path(data: &mut Vec<u8>, path: &[PathSegment]) {
    for seg in path {
        canonical_part(data, seg);
    }
}

// Does NOT guarantee a unique output for every unique input
// e.g. ['a.b'] and ['a','b'] both return 'a.b'.
pub(crate) fn path_to_string(path: &[PathSegment]) -> String {
    let mut ret = String::new();
    for seg in path {
        if !ret.is_empty() {
            ret.push('.');
        }
        ret.push_str(seg.key());
    }
    ret
}
