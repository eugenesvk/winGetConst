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

pub fn generate_rustdoc() {
  // 1. install semver checks to generate json
  // cargo binstall cargo-semver-checks
  // 2. generate json for the latest Windows sys crate version
  // cargo semver-checks --manifest-path r'path/to/cargo/registry/src/github.com-1ecc6299db9ec823/windows-sys-0.48.0/Cargo.toml' --baseline-version 0.45.0
  // 3. use rustdoc_find_consts_adapter_directly to parse that json

  // ↓ simpler way to generate docs with cargo?
  // generate rustdoc pub_module_level_const_missing.json (though crate creates a new project first to do that)
  // $RUSTC_BOOTSTRAP=1
  // $RUSTDOCFLAGS="-Z unstable-options --document-private-items --document-hidden-items --output-format=json --cap-lints allow"
  // $manifest = "../test_crates/pub_module_level_const_missing/new/Cargo.toml"
  // $target = "..cargo/semver-checks/target"
  // $pkg = "pub_module_level_const_missing@0.1.1"
  // cargo doc --manifest-path $manifest --target-dir $target --package $pkg
}

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


pub fn rem_first(value: &str) -> &str {
  let mut chars = value.chars();
  chars.next();
  // chars.next_back();
  chars.as_str()
}
pub fn parse_lit(num:&str) -> String {
  use litrs::{Literal};
  let mut sign   	= "".to_string();
  let     nonum  	= "✗".to_string();
  let mut num_pos	= num.to_string();
  if num.starts_with("-") {
    sign   	+= "-";
    num_pos	 = rem_first(&num).to_string();
  }
  let lit_try	= Literal::parse(num_pos); //.expect(&format!("✗parse literal ‘{}’",&num));
  match lit_try {
    Ok(lit) => { match lit {
      // 27f32 bugs as an integer https://github.com/LukasKalbertodt/litrs/issues/14
      Literal::Integer	(lit) => { //https://doc.rust-lang.org/stable/std/primitive/index.html
        // avoids matching by suffix, use max Unsigned value to avoid overflowing of -MAXu128
        match lit.value::<u128>().map(|n| n.to_string()) {Some(n) => sign + &n.to_string(), None => nonum}

        // match lit.value::<u128>()	{Some(n) => sign + &n.to_string(), None => nonum},
        // p!("Integer raw_input {:?}",lit.raw_input());
        // todo: change to proper types one weird Rust bug can be avoided witn a .lenient version
        // https://docs.rs/litrs/latest/litrs/struct.IntegerLit.html
        /*
        match lit.suffix() {
           // ↓ would become floats?
           "f32"	=> match lit.value::< u32>()	{Some(n) => sign + &n.to_string(), None => nonum},
           "f64"	=> match lit.value::< u64>()	{Some(n) => sign + &n.to_string(), None => nonum},
           // ↓ −128i8 would overflow without the sing: 128i8 doesn't fit the max 127i8
         //  "i8"	=> match lit.value::<  i8>() 	{Some(n) => sign + &n.to_string(), None => nonum},
         // "i16"	=> match lit.value::< i16>() 	{Some(n) => sign + &n.to_string(), None => nonum},
         // "i32"	=> match lit.value::< i32>() 	{Some(n) => sign + &n.to_string(), None => nonum},
         // "i64"	=> match lit.value::< i64>() 	{Some(n) => sign + &n.to_string(), None => nonum},
            "i8" 	=> match lit.value::<  u8>() 	{Some(n) => sign + &n.to_string(), None => nonum},
           "i16" 	=> match lit.value::< u16>() 	{Some(n) => sign + &n.to_string(), None => nonum},
           "i32" 	=> match lit.value::< u32>() 	{Some(n) => sign + &n.to_string(), None => nonum},
           "i64" 	=> match lit.value::< u64>() 	{Some(n) => sign + &n.to_string(), None => nonum},
          "i128" 	=> match lit.value::<u128>() 	{Some(n) => sign + &n.to_string(), None => nonum},
          "isize"	=> match lit.value::<usize>()	{Some(n) => sign + &n.to_string(), None => nonum},
            "u8" 	=> match lit.value::<  u8>() 	{Some(n) => sign + &n.to_string(), None => nonum},
           "u16" 	=> match lit.value::< u16>() 	{Some(n) => sign + &n.to_string(), None => nonum},
           "u32" 	=> match lit.value::< u32>() 	{Some(n) => sign + &n.to_string(), None => nonum},
           "u64" 	=> match lit.value::< u64>() 	{Some(n) => sign + &n.to_string(), None => nonum},
          "u128" 	=> match lit.value::<u128>() 	{Some(n) => sign + &n.to_string(), None => nonum},
          "usize"	=> match lit.value::<usize>()	{Some(n) => sign + &n.to_string(), None => nonum},
          ""     	=> match lit.value::< u64>() 	{Some(n) => sign + &n.to_string(), None => nonum},
          // _   	=> match lit.value::<u128>() 	{Some(n) => sign + &n.to_string(), None => nonum},
          _      	=> nonum,
        }*/
      }
      Literal::Float	(lit) => {
        // p!("Float part {:?}",lit.number_part());
        match lit.suffix() {
          "f32"	=> match lit.number_part().parse::<f32>()	{Ok(n) => sign + &n.to_string(), Err(_) => nonum},
          "f64"	=> match lit.number_part().parse::<f64>()	{Ok(n) => sign + &n.to_string(), Err(_) => nonum},
          ""   	=> match lit.number_part().parse::<f64>()	{Ok(n) => sign + &n.to_string(), Err(_) => nonum},
          _    	=> nonum,
        }}
      Literal::Bool      	(lit) => { lit.to_string() }
      Literal::Char      	(lit) => { lit.to_string() }
      Literal::String    	(lit) => { lit.to_string() }
      Literal::Byte      	(lit) => { lit.to_string() }
      Literal::ByteString	(lit) => { lit.to_string() }
    }},
    Err(ParseError) => "_".to_string(),
  }
}
fn parse_lit_examples() { // BUGS with 27f32, parsed as integer
  let mylit	="5_000_000i64" 	; p!("{:?} \t→ {:?}",mylit,parse_lit(mylit));
  let mylit	="-5_000_000i64"	; p!("{:?} \t→ {:?}",mylit,parse_lit(mylit));
  let mylit	="2.1f32"       	; p!("{:?} \t→ {:?}",mylit,parse_lit(mylit));
  let mylit	="-2.1f32"      	; p!("{:?} \t→ {:?}",mylit,parse_lit(mylit));
  let mylit	="32f32"        	; p!("{:?} \t→ {:?}",mylit,parse_lit(mylit));
  let mylit	="-32f32"       	; p!("{:?} \t→ {:?}",mylit,parse_lit(mylit));
  // p!("{:?}","33".parse::<f32>());
  // use litrs::{FloatLit};
  // p!("{:?}",FloatLit::parse("27.0f32").expect("failed to parse as float literal"));
  // p!("{:?}",FloatLit::parse("20f32").expect("failed to parse as float literal"));
}

fn parse_num_suffix(num:&str) -> Result<String,()> {
  use fancy_regex::Regex;
  const num_type :&str	= r#"(?x:
  (?<num>[0-9_]+)
  (?<typ>.+)
  )"#;
  let re      	= Regex::new(num_type).unwrap();
  let result  	= re.captures(num); // on 5i32
  let captures	= result.expect("Error running regex");
  match captures {
    Some(groups)  	=> {match groups.name("num") { // 5
      Some(gmatch)	=> Ok(gmatch.as_str().to_string()),
      None        	=> Err(()),}},
    None          	=> Err(()),
  }
}



pub fn get_const_kv_from(src:&Path) -> Result<HashMap<String,String>,Box<dyn std::error::Error>> {
  let mut win32_const:HashMap<String,String>	= HashMap::with_capacity(200_000 * 2);

  if let Ok(lines) = read_lines(src) {
    for line in lines { // consumes iterator, returns an (Optional) String
      if let Ok(val_tab_key) = line {	// WM_RENDERFORMAT 773
        let val_key:Vec<&str> = val_tab_key.splitn(3,'\t').collect();
        if val_key.len() >= 2 {
          let (key,val)	= (val_key[0].to_string(),val_key[1].to_string()); //WM_RENDERFORMAT 773
          // p!("{}={}",&key,&val);
          win32_const.insert(key, val); // push original WM_RENDERFORMAT
        }
      }
    }
  }
  Ok(win32_const)
}

pub const tab	:&[u8]	= "\t".as_bytes();
pub const nl 	:&[u8]	= "\n".as_bytes();
use std::fs  	::File;
use std::io  	::{self,prelude::*,BufRead,BufWriter};
fn compare_this_to_ziggle() {
  let ziggle_p          	:&Path	= Path::new("../winAPIconst/data/ziggle_clean64.txt");
  let this_p            	:&Path	= Path::new("./winConst_Valid.txt");
  let this_blank_p      	:&Path	= Path::new("./winConst_Blank.txt");
  let log_p1            	:&Path	= Path::new("./winConst_vs_ziggle_extra.log");
  let log_p2            	:&Path	= Path::new("./winConst_vs_ziggle_missing.log");
  let log_p3            	:&Path	= Path::new("./winConst_vs_ziggle_blank.log");
  let log_p4            	:&Path	= Path::new("./winConst_vs_ziggle_diff_value.log");
  // if log_p1.is_file()	{return Err(format!("Aborting, file exists {:?}",log_p1).into())};
  // if log_p2.is_file()	{return Err(format!("Aborting, file exists {:?}",log_p2).into())};
  // if log_p3.is_file()	{return Err(format!("Aborting, file exists {:?}",log_p3).into())};
  // if log_p4.is_file()	{return Err(format!("Aborting, file exists {:?}",log_p4).into())};
  let log_f1            	= File::create (&log_p1).unwrap();
  let mut file_log_buff1	= BufWriter::new(log_f1);
  let log_f2            	= File::create (&log_p2).unwrap();
  let mut file_log_buff2	= BufWriter::new(log_f2);
  let log_f3            	= File::create (&log_p3).unwrap();
  let mut file_log_buff3	= BufWriter::new(log_f3);
  let log_f4            	= File::create (&log_p4).unwrap();
  let mut file_log_buff4	= BufWriter::new(log_f4);

  // file_log_buff1.write("Constants present in this crate, but missing from Ziggle".as_bytes()).unwrap();
  // file_log_buff1.write(nl).unwrap();
  // file_log_buff2.write("Constants present in Ziggle, but missing from this crate".as_bytes()).unwrap();
  // file_log_buff2.write(nl).unwrap();
  // file_log_buff3.write("Constants present in Ziggle, but blank in this crate".as_bytes()).unwrap();
  // file_log_buff3.write(nl).unwrap();
  // file_log_buff4.write("Constants present in Ziggle and this crate, but with different values".as_bytes()).unwrap();
  // file_log_buff4.write(nl).unwrap();

  let const_ziggle    	:HashMap<String,String> = get_const_kv_from(ziggle_p      ).unwrap();
  let const_this      	:HashMap<String,String> = get_const_kv_from(this_p        ).unwrap();
  let const_this_blank	:HashMap<String,String> = get_const_kv_from(this_blank_p  ).unwrap();
  for (c_name,c_val) in &const_this   {
    if   !const_ziggle    .contains_key(c_name) {file_log_buff1.write(format!("{}\t{}\n"    ,c_name,c_val).as_bytes()).unwrap();
    } else if &const_ziggle[c_name] != c_val    {file_log_buff4.write(format!("{}\t{}\t{}\n",c_name,c_val,const_ziggle[c_name]).as_bytes()).unwrap();}
  };
  for (c_name,c_val) in &const_ziggle {
    if   !const_this      .contains_key(c_name)	{
      if !const_this_blank.contains_key(c_name)	{file_log_buff2.write(format!("{}\t{}\n"    ,c_name,c_val).as_bytes()).unwrap();
      } else                                   	{file_log_buff3.write(format!("{}\t{}\n"    ,c_name,c_val).as_bytes()).unwrap();};};};
  file_log_buff1.flush().unwrap();
  file_log_buff2.flush().unwrap();
  file_log_buff3.flush().unwrap();
  file_log_buff4.flush().unwrap();
}

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
  // let val:&str	= "-2147483648i32";
  let val:&str   	= "-170_141_183_460_469_231_731_687_303_715_884_105_728i128";
  let val:&str   	= "-9_223_372_036_854_775_808i64";
  let my_num     	= parse_lit(&val);
  p!("{:?} → {:?}",val,my_num);
}

fn main() {
  // 1 Parses Windows_sys crate rustdocs generated via cargo-semver-checks and saves results to a simple tab-separated name⭾value⭾type file
    // winConst_All.txt  	all constants
    // winConst_Blank.txt	constants where values are blank (_) as they're not in the rustdocs
    // winConst_Valid.txt	all non-blank constants
  let crate_rustdoc_path 	:&Path	= Path::new("./test_data/pub_module_level_const_missing_mod.json"); // short crate to use for testing instead of the huge ↓
  let crate_rustdoc_path 	:&Path	= Path::new("./test_data/windows_sys_0.48.0.json");
  let query_path         	:&Path	= Path::new("./test_data/query_const.ron");
  // let _ = rustdoc_find_consts_adapter_directly(&crate_rustdoc_path,&query_path);

  // 2 Compares winConst files ↑ to a ziggle database and generates lists of differences (extra constants, missing constants, constants with different values)
  // compare_this_to_ziggle();

  // 3 Add missing constants from the ziggle database
  // merge_this_with_ziggle();

  // test1()
}
