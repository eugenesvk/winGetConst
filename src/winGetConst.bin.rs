#![allow(unused_imports,unused_variables,unreachable_code,dead_code,non_upper_case_globals,non_snake_case)]
extern crate helper;
use helper        	::*; // gets macros
use helper::alias 	::*;
use helper::helper	::*;
use helper::fs    	::*;

use std::path       	::{self,Path,PathBuf};
use std::collections	::{HashMap,BTreeMap};

pub mod binmod;
use crate::binmod::print42;


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

fn test(){
  pub enum TypeX {
    Generic(String),
    Array {
      t: String,
      len: String,
    },
  }
  let mytype1 = TypeX::Generic("asdf".to_string());
  let mytype2 = TypeX::Array{t:"array_t".to_string(),len:"array_len".to_string()};
  let out1 = match mytype1 {
    TypeX::Generic(s)	=> s,
    _                	=> "something else".to_string(),
  };
  let out2 = match mytype2 {
    TypeX::Generic(s) 	=> s,
    TypeX::Array{t,..}	=> t,
  };
  p!("{}\t{}",out1,out2);
}



  rustdoc_find_consts_adapter_directly(&crate_rustdoc_path,&query_path);

fn test1(){
  // let val:&str	= "5i32";
  // let my_num  	= parse_num_suffix(&val);
  // p!("{:?} → {:?}",val,my_num.unwrap());
  // const LOG_AS  : &str = "batch"  ; // true None `"\"batch\""` including escaped quotes
  let val:&str	= "-2147483648i32";
  let my_num  	= parse_lit(&val);
  p!("{:?} → {:?}",val,my_num);
}
}
