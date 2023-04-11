// SPDX-FileCopyrightText: Charles Barto
//
// SPDX-License-Identifier: LGPL-3.0-only

#![feature(ptr_metadata)]
#![feature(variant_count)]
#![feature(map_try_insert)]
#![feature(new_uninit)]
#![feature(read_buf)]

#[cfg(windows)]
mod xattr_util;
pub mod repo;


pub mod mutable_tree;
pub mod perms;
pub use crate::repo::*;
