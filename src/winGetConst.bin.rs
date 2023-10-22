#![allow(unused_imports,unused_variables,unreachable_code,dead_code,non_upper_case_globals,non_snake_case)]
extern crate helper;
use helper        	::*; // gets macros
use helper::alias 	::*;
use helper::helper	::*;
use helper::fs    	::*;
use helper::parser	::*;

use std::path       	::{self,Path,PathBuf};
use std::collections	::{HashMap,BTreeMap,HashSet};

pub mod binmod;
use crate::binmod::print42;


use serde::{Deserialize, Serialize};
use anyhow::Context;




use std::sync::Arc;
use trustfall::{Schema, TryIntoStruct};

use std::{env,iter::Peekable,time::Duration,error::Error,process,
  ffi::OsString,};

pub mod trustfall_rustdoc;
use trustfall_rustdoc::rustdoc_find_consts;

pub fn generate_rustdoc() {
  // 1. install semver checks to generate json
  // cargo binstall cargo-semver-checks
  // 2. generate json for the latest Windows sys crate version
  // manifest_p = r'path/to/cargo/registry/src/github.com-1ecc6299db9ec823/windows-sys-0.48.0/Cargo.toml'
  // cargo semver-checks --manifest-path @(manifest_p) --baseline-root @(manifest_p)
  // get results from ./cache/cargo/semver-checks/target/doc
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

pub const col_name_nm     	:&str	= "name";
pub const col_value_nm    	:&str	= "value";
pub const col_namespace_nm	:&str	= "namespace";

pub fn get_const_kv_from(src:&Path) -> Result<(HashMap<String,String>,HashMap<String,String>),Box<dyn Error>> {
  p!("Processing ‘{:?}’ into a HashMap...",&src);
  let mut win32_const:HashMap<String,String>	= HashMap::with_capacity(200_000 * 2);
  let mut dupe_const :HashMap<String,String>	= HashMap::with_capacity(    100 * 2);
  let mut dupe_set   :HashSet<String>       	= HashSet::new();

  let mut rdr	= csv::ReaderBuilder::new().has_headers(true).delimiter(b'\t').comment(Some(b'#')).from_path(src)?;
  let hd = rdr.headers()?.clone();
  let col_name_i      	= hd.iter().position(|x| x.to_ascii_lowercase() == col_name_nm     ).unwrap();
  let col_value_i     	= hd.iter().position(|x| x.to_ascii_lowercase() == col_value_nm    ).unwrap();
  let col_namespace_i_	= hd.iter().position(|x| x.to_ascii_lowercase() == col_namespace_nm);

  use unescaper::unescape; // required since strings are escaped
  for (i, res) in rdr.records().enumerate() {
    let record = res?;
    let (key,val)	= (record[col_name_i ].to_string(),unescape(&record[col_value_i])?); //WM_RENDERFORMAT 773
    let mut ns = "".to_string();
    if let Some(col_namespace_i)	= col_namespace_i_ {
      ns = ns + " @ " + &record[col_namespace_i];}
    if i==1 {p!("  {}={}{}",&key,&val,&ns);}
    if win32_const.contains_key(&key) &&
       win32_const.get(&key).unwrap() != &val	{//p!("  dupe {}={}{}",&key,&val,&ns);
                                             	 win32_const.remove(&key);
                                             	 dupe_set   .insert(key);
    } else                                   	{win32_const.insert(key,val);}
  }
  if dupe_set.len() != 0 { // repeat iteration to insert only dupes, but now with namespaces
    p!("  reinserting dupes");
    let mut rdr	= csv::ReaderBuilder::new().has_headers(true).delimiter(b'\t').comment(Some(b'#')).from_path(src)?;
    let mut print_1 = true;
    for (i, res) in rdr.records().enumerate() {
      let record = res?;
      let (key,val)	= (record[col_name_i ].to_string(),unescape(&record[col_value_i])?); //WM_RENDERFORMAT 773
      if dupe_set.contains(&key) {
        let mut ns = "".to_string();
        if let Some(col_namespace_i)	= col_namespace_i_ {
          ns = ns + "@" + &record[col_namespace_i];
        } else {p!("dupe, but no namespace, overwriting {}",&key);}
        let key_ns = key.clone() + &ns;
        if print_1 {p!("  dupe {}={}",&key_ns,&val); print_1 = false}
        win32_const.insert(key_ns.clone(),val.clone());
        dupe_const .insert(key_ns        ,val);}
    }
  }

  p!("  → HashMap of ‘{}’ elements and ‘{}’ dupes from ‘{:?}’",win32_const.len(),dupe_const.len(),&src);
  Ok((win32_const,dupe_const))
}

pub const tab	:&[u8]	= "\t".as_bytes();
pub const nl 	:&[u8]	= "\n".as_bytes();
use std::fs  	::File;
use std::io  	::{self,prelude::*,BufRead,BufWriter};
fn compare_this_to_ziggle(p_i_zig:(&Path,u8),p_i_this:(&Path,u8),p_i_blank:Option<(&Path,u8)>) {
  let log_p1            	:&Path	= Path::new("./data/winConst_vs_ziggle_extra.log");
  let log_p2            	:&Path	= Path::new("./data/winConst_vs_ziggle_missing.log");
  let log_p4            	:&Path	= Path::new("./data/winConst_vs_ziggle_diff_value.log");
  // if log_p1.is_file()	{return Err(format!("Aborting, file exists {:?}",log_p1).into())};
  // if log_p2.is_file()	{return Err(format!("Aborting, file exists {:?}",log_p2).into())};
  // if log_p4.is_file()	{return Err(format!("Aborting, file exists {:?}",log_p4).into())};
  let log_f1            	= File::create (&log_p1).unwrap();
  let mut log_buff1     	= BufWriter::new(log_f1);
  let log_f2            	= File::create (&log_p2).unwrap();
  let mut log_buff2     	= BufWriter::new(log_f2);
  let log_f4            	= File::create (&log_p4).unwrap();
  let mut log_buff4     	= BufWriter::new(log_f4);

  // log_buff1.write("Constants present in this crate, but missing from Ziggle".as_bytes()).unwrap();
  // log_buff1.write(nl).unwrap();
  // log_buff2.write("Constants present in Ziggle, but missing from this crate".as_bytes()).unwrap();
  // log_buff2.write(nl).unwrap();
  // log_buff4.write("Constants present in Ziggle and this crate, but with different values".as_bytes()).unwrap();
  // log_buff4.write(nl).unwrap();

  let col_i_this  	:u8	= p_i_this.1;
  let col_i_zig   	:u8	= p_i_zig.1;
  let mut opt_this	= SearchOpts::default(); opt_this.add_option(SearchOpt::ValInd(col_i_this));
  let mut opt_zig 	= SearchOpts::default(); opt_zig .add_option(SearchOpt::ValInd(col_i_zig ));
  let this_p      	:&Path	= p_i_this.0;
  let ziggle_p    	:&Path	= p_i_zig.0;
  let const_zig   	:HashMap<String,String> = get_const_kv_from(ziggle_p,&opt_zig ).unwrap();
  let const_this  	:HashMap<String,String> = get_const_kv_from(this_p  ,&opt_this).unwrap();
  for (c_name,c_val) in &const_this   {
    if   !const_zig    .contains_key(c_name) {log_buff1.write(format!("{}\t{}\n"    ,c_name,c_val).as_bytes()).unwrap();
    } else if &const_zig[c_name] != c_val    {log_buff4.write(format!("{}\t{}\t{}\n",c_name,c_val,const_zig[c_name]).as_bytes()).unwrap();}
  };
  log_buff1.flush().unwrap();
  log_buff2.flush().unwrap();
  log_buff4.flush().unwrap();

  match p_i_blank {
    Some((blank_p,blank_col_i)) => {
      let mut sopts_blank   	= SearchOpts::default(); sopts_blank.add_option(SearchOpt::ValInd(col_i_this  ));
      let log_p3:&Path      	= Path::new("./data/winConst_vs_ziggle_blank.log");
      // if log_p3.is_file()	{return Err(format!("Aborting, file exists {:?}",log_p3).into())};
      let log_blank_f       	= File::create (&log_p3).unwrap();
      let mut log_blank_buff	= BufWriter::new(log_blank_f);
      // log_blank_buff.write("Constants present in Ziggle, but blank in this crate".as_bytes()).unwrap();
      // log_blank_buff.write(nl).unwrap();
      let const_this_blank	:HashMap<String,String> = get_const_kv_from(blank_p,&sopts_blank).unwrap();
      for (c_name,c_val) in &const_zig {
        if   !const_this      .contains_key(c_name)	{
          if !const_this_blank.contains_key(c_name)	{log_buff2.write(format!("{}\t{}\n"    ,c_name,c_val).as_bytes()).unwrap();
          } else                                   	{log_blank_buff.write(format!("{}\t{}\n"    ,c_name,c_val).as_bytes()).unwrap();};};};
      log_blank_buff.flush().unwrap();
    },
    None	=> {}}
}

fn merge_this_with_ziggle(col_i:u8) {
  let ziggle_p       	:&Path	= Path::new("../winAPIconst/data/ziggle_clean64.txt");
  let this_p         	:&Path	= Path::new("./data/winConst_Valid.txt");
  let merged_p       	:&Path	= Path::new("./data/winConst_Valid_ziggle.txt");
  let merged_f       	= File::create(&merged_p).unwrap();
  let mut merged_buff	= BufWriter::new(merged_f);

  let mut sopts = SearchOpts::default(); sopts.add_option(SearchOpt::ValInd(col_i));
  let const_zig      	:HashMap<String,String> = get_const_kv_from(ziggle_p,&sopts).unwrap();
  let const_this     	:HashMap<String,String> = get_const_kv_from(this_p  ,&sopts).unwrap();
  let mut win32_const	:BTreeMap<String,String>	= BTreeMap::new();
  for (c_name,c_val) in &const_this {
    win32_const.insert(c_name.to_string(),c_val.to_string());};
  for (c_name,c_val) in &const_zig {
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

fn winmd_to_tsv(args:&Vec<String>) { /*! aka 'riddle': parse Windows Metadata (WinMD) files using [windows-bindgen](https://crates.io/crates/windows-bindgen) compiler crate and saves results to a simple tab-separated name⭾type⭾type_native⭾value file */
  // let args: Vec<String> = std::env::args().skip(1).collect();
  if args.is_empty() {println!(r#"Usage: riddle.exe [options...]
Options:
  --in  <path>          Path to files and directories containing .winmd and .rdl files
  --out <path>          Path to .winmd, .rdl, or .rs file to generate
  --filter <namespace>  Namespaces to include or !exclude in output
  --config <key=value>  Override a configuration value
  --format              Format .rdl files only
  --etc <path>          File containing command line options"#);
  } else {
    match windows_bindgen::bindgen(args) {
      Ok(ok)    	=> p!("{}", ok),
      Err(error)	=> {eprintln!("{}",error); std::process::exit(1);}}
  }
}

fn rustdocs_to_tsv() { /*! Parse Windows_sys crate rustdocs generated via cargo-semver-checks and saves results to a simple tab-separated name⭾value⭾type file
    winConst_All.txt    	all constants
    winConst_Blank.txt  	constants where values are blank (_) as they're not in the rustdocs
    winConst_Valid.txt  	all non-blank constants */
  let crate_rustdoc_path	:&Path	= Path::new("./test_data/pub_module_level_const_missing_mod.json"); // short crate to use for testing instead of the huge ↓
  let crate_rustdoc_path	:&Path	= Path::new("./test_data/windows_sys_0.48.0.json");
  let query_path        	:&Path	= Path::new("./src/query/query_const.ron");
  let _ = rustdoc_find_consts(&crate_rustdoc_path,&query_path);
}

fn main() {
  let mut args: Vec<String> = std::env::args().skip(1).collect();
  let ziggle_p	:&Path	= Path::new("../winAPIconst/data/ziggle_clean64.txt");
  if        let Some(pos) = args.iter().position(|x| *x == "wmd") {
    args.remove(pos);
    p!("converting WinMD to TSV");
    winmd_to_tsv(&args); // 1 WinMD → TSV
    _ = r#"
    fout  	= r'./test_data/win_bindgen'
    filter	= r'Windows.Wdk.System.Registry'
    cargo run -- --out @(fout) --filter @(filter)
    // ↓ not needed since 'metadata' feature already includes all .winmd files
    fin	= r'./win/libs/bindgen/default/Windows.Wdk.winmd'
    cargo run -- --in @(fin) --out @(fout) --filter @(filter)
    "#;
  } else if let Some(pos) = args.iter().position(|x| *x == "rdoc") {
    p!("Comparing WinMD to Ziggle");
    // compare_winmd_to_ziggle();	// 2 Compares winConst files ↑ to a ziggle database and generates lists of differences (extra constants, missing constants, constants with different values)
  } else if let Some(pos) = args.iter().position(|x| *x == "rdoc") {
    p!("converting Windows_sys rustdocs to TSV");
    rustdocs_to_tsv();	// 1 Windows_sys rustdocs → TSV
  } else if let Some(pos) = args.iter().position(|x| *x == "wmd2ziggle") {
    let this_p:&Path	= Path::new("./data/winConst_bindgen_All_185k");
    compare_this_to_ziggle((ziggle_p,2),(this_p,4),None); // 2 Compares winConst files ↑ to a ziggle database and generates lists of differences (extra constants, missing constants, constants with different values)
  } else if let Some(pos) = args.iter().position(|x| *x == "rdoc2ziggle") {
    let this_p      	:&Path	= Path::new("./data/winConst_Valid.txt");
    let this_blank_p	:&Path	= Path::new("./data/winConst_Blank.txt");
    compare_this_to_ziggle((ziggle_p,2),(this_p,2),Some((this_blank_p,2))); // 2 Compares winConst files ↑ to a ziggle database and generates lists of differences (extra constants, missing constants, constants with different values)
  } else if let Some(pos) = args.iter().position(|x| *x == "merge2ziggle") {
    let col_i = 2;
    merge_this_with_ziggle(col_i);	// 3 Add missing constants from the ziggle database
  } else if let Some(pos) = args.iter().position(|x| *x == "test") {
    // test1()
    // let tsv		= "21\t22\t23".to_string();
    // let result = get_col_val(tsv,SearchOpts::default().add_option(SearchOpt::ValInd(2)));
    // p!("{:?}",result);
  }
}
