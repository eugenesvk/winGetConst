#![allow(unused_imports,unused_variables,unreachable_code,dead_code,non_upper_case_globals,non_snake_case)]
extern crate helper;
use std::path::PathBuf;
use helper        	::*; // gets macros
use helper::alias 	::*;
use helper::helper	::*;

pub mod binmod;
use crate::binmod::print42;

use std::{collections::BTreeMap, path::Path};

use serde::{Deserialize, Serialize};
use anyhow::Context;
use trustfall::{FieldValue,TransparentValue};




use std::sync::Arc;
use trustfall::{Schema, TryIntoStruct};

use std::{env,io::Write,iter::Peekable,time::Duration};

pub mod trustfall_rustdoc_old;
use trustfall_rustdoc_old::rustdoc_find_consts_adapter_directly;

fn main() {
  // check_crate().unwrap();
  let crate_rustdoc_path	:&Path	= Path::new("./test_data/pub_module_level_const_missing_mod.json");
  let query_path        	:&Path	= Path::new("./test_data/query_const.ron");
  // let _ = rustdoc_find_consts(&crate_rustdoc_path,&query_path);

  rustdoc_find_consts_adapter_directly(&crate_rustdoc_path,&query_path);
}
