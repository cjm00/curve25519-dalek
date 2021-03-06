// -*- mode: rust; -*-
//
// This file is part of curve25519-dalek.
// Copyright (c) 2016-2017 Isis Lovecruft, Henry de Valence
// See LICENSE for licensing information.
//
// Authors:
// - Isis Agora Lovecruft <isis@patternsinthevoid.net>
// - Henry de Valence <hdevalence@hdevalence.ca>

//! Miscellaneous common utility functions.

/// Convert an array of (at least) three bytes into an i64.
#[inline]
//#[allow(dead_code)]
pub fn load3(input: &[u8]) -> i64 {
       (input[0] as i64)
    | ((input[1] as i64) << 8)
    | ((input[2] as i64) << 16)
}

/// Convert an array of (at least) four bytes into an i64.
#[inline]
//#[allow(dead_code)]
pub fn load4(input: &[u8]) -> i64 {
       (input[0] as i64)
    | ((input[1] as i64) << 8)
    | ((input[2] as i64) << 16)
    | ((input[3] as i64) << 24)
}

/// Convert an array of (at least) eight bytes into a u64.
#[inline]
//#[allow(dead_code)]
pub fn load8(input: &[u8]) -> u64 {
       (input[0] as u64)
    | ((input[1] as u64) << 8)
    | ((input[2] as u64) << 16)
    | ((input[3] as u64) << 24)
    | ((input[4] as u64) << 32)
    | ((input[5] as u64) << 40)
    | ((input[6] as u64) << 48)
    | ((input[7] as u64) << 56)
}
