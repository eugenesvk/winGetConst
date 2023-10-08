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

fn merge_this_with_ziggle() {
  let ziggle_p       	:&Path	= Path::new("../winAPIconst/data/ziggle_clean64.txt");
  let this_p         	:&Path	= Path::new("./data/winConst_Valid.txt");
  let merged_p       	:&Path	= Path::new("./data/winConst_Valid_ziggle.txt");
  let merged_f       	= File::create(&merged_p).unwrap();
  let mut merged_buff	= BufWriter::new(merged_f);

  let const_ziggle   	:HashMap<String,String> = get_const_kv_from(ziggle_p).unwrap();
  let const_this     	:HashMap<String,String> = get_const_kv_from(this_p  ).unwrap();
  let mut win32_const	:BTreeMap<String,String>	= BTreeMap::new();
  for (c_name,c_val) in &const_this {
    win32_const.insert(c_name.to_string(),c_val.to_string());};
  for (c_name,c_val) in &const_ziggle {
    if   !const_this.contains_key(c_name)	{win32_const.insert(c_name.to_string(),c_val.to_string());}};
  for (c_name,c_val) in &win32_const {
    merged_buff.write(format!("{}\t{}\n",c_name,c_val).as_bytes()).unwrap();};
  merged_buff.flush().unwrap();
}

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
