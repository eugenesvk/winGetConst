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

use std::{env,iter::Peekable,time::Duration,error::Error,process,
  ffi::{OsString,OsStr}};

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
pub fn get_const_kvals_from(src:&Path) -> Result<(BTreeMap<String,Vec<String>>,BTreeMap<String,Vec<String>>),Box<dyn Error>> {
  p!("Processing ‘{:?}’ into a sorted BTreeMap...",&src);
  let mut win32_const:BTreeMap<String,Vec<String>>	= BTreeMap::new();
  let mut dupe_const :BTreeMap<String,Vec<String>>	= BTreeMap::new();
  let mut dupe_set   :HashSet<String>             	= HashSet::new();

  let mut rdr	= csv::ReaderBuilder::new().has_headers(true).delimiter(b'\t').comment(Some(b'#')).from_path(src)?;
  let hd = rdr.headers()?.clone();
  let col_name_i      	= hd.iter().position(|x| x.to_ascii_lowercase() == col_name_nm     ).unwrap();
  let col_value_i     	= hd.iter().position(|x| x.to_ascii_lowercase() == col_value_nm    ).unwrap();
  let col_namespace_i_	= hd.iter().position(|x| x.to_ascii_lowercase() == col_namespace_nm);

  use unescaper::unescape; // required since strings are escaped
  for (i, res) in rdr.records().enumerate() {
    let record = res?;
    let (key,val)	= (record[col_name_i ].to_string(),unescape(&record[col_value_i])?); //WM_RENDERFORMAT 773
    if win32_const.contains_key(&key) &&
       win32_const.get(&key).unwrap()[col_value_i] != val {
      win32_const.remove(&key);
      dupe_set   .insert(key); // ↓ store full record use key only for sorting
    } else	{
      let rec_vec = record.iter().map(|x| x.to_string()).collect::<Vec<String>>();
      win32_const.insert(key,rec_vec);}
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
        let mut rec_vec = record.iter().map(|x| x.to_string()).collect::<Vec<String>>();
        rec_vec[0] = key_ns.clone();
        win32_const.insert(key_ns.clone(),rec_vec.clone());
        dupe_const .insert(key_ns        ,rec_vec);}
    }
  }

  p!("  → BTreeMap of ‘{}’ elements and ‘{}’ dupes from ‘{:?}’",win32_const.len(),dupe_const.len(),&src);
  Ok((win32_const,dupe_const))
}


pub const tab	:&[u8]	= "\t".as_bytes();
pub const nl 	:&[u8]	= "\n".as_bytes();
use std::fs  	::File;
use std::io  	::{self,prelude::*,BufRead,BufWriter,Write};
fn compare_this_to_ziggle(ziggle_p:&Path,this_p:&Path,blank_p_:Option<&Path>) {
  let log_extra_p  	:&Path	= Path::new("./data/winConst_vs_ziggle_extra.log");
  let log_miss_p   	:&Path	= Path::new("./data/winConst_vs_ziggle_missing.log");
  let log_diff_p   	:&Path	= Path::new("./data/winConst_vs_ziggle_diff_value.log");
  let log_dupe_z_p 	:&Path	= Path::new("./data/winConst_dupe_ziggle.log");
  let log_dupe_th_p	:&Path	= Path::new("./data/winConst_dupe.log");

  // if log_extra_p.is_file()  	{return Err(format!("Aborting, file exists {:?}",log_extra_p  	).into())};
  // if log_miss_p.is_file()   	{return Err(format!("Aborting, file exists {:?}",log_miss_p   	).into())};
  // if log_diff_p.is_file()   	{return Err(format!("Aborting, file exists {:?}",log_diff_p   	).into())};
  // if log_dupe_z_p.is_file() 	{return Err(format!("Aborting, file exists {:?}",log_dupe_z_p 	).into())};
  // if log_dupe_th_p.is_file()	{return Err(format!("Aborting, file exists {:?}",log_dupe_th_p	).into())};

  let log_extra_f  	= File::create(&log_extra_p  	).unwrap();	let mut log_extra_buff  	= BufWriter::new(log_extra_f);
  let log_miss_f   	= File::create(&log_miss_p   	).unwrap();	let mut log_miss_buff   	= BufWriter::new(log_miss_f);
  let log_diff_f   	= File::create(&log_diff_p   	).unwrap();	let mut log_diff_buff   	= BufWriter::new(log_diff_f);
  let log_dupe_z_f 	= File::create(&log_dupe_z_p 	).unwrap();	let mut log_dupe_z_buff 	= BufWriter::new(log_dupe_z_f);
  let log_dupe_th_f	= File::create(&log_dupe_th_p	).unwrap();	let mut log_dupe_th_buff	= BufWriter::new(log_dupe_th_f);

  let header     	= "Name\tValue\n".as_bytes();
  let header_diff	= "Name\tValue\tValueZig\n".as_bytes();
  log_extra_buff.write("# Constants present in this crate, but missing from Ziggle\n".as_bytes()).unwrap();
  log_extra_buff.write(&header).unwrap();
  log_miss_buff.write("# Constants present in Ziggle, but missing from this crate\n".as_bytes()).unwrap();
  log_miss_buff.write(&header).unwrap();
  log_diff_buff.write("# Constants present in Ziggle and this crate, but with different values\n".as_bytes()).unwrap();
  log_diff_buff.write(&header_diff).unwrap();
  log_dupe_z_buff.write("# Duplicate constants in Ziggle\n".as_bytes()).unwrap();
  log_dupe_z_buff.write(&header).unwrap();
  log_dupe_th_buff.write("# Duplicate constants in this crate\n".as_bytes()).unwrap();
  log_dupe_th_buff.write(&header).unwrap();

  let (const_zig ,dupe_zig )	= get_const_kv_from(ziggle_p).unwrap();
  let (const_this,dupe_this)	= get_const_kv_from(this_p  ).unwrap();
  for (c_name,c_val) in &const_this   {
    if !const_zig.contains_key(c_name)   	{log_extra_buff.write(format!("{}\t{}\n"    ,c_name,c_val).as_bytes()).unwrap();
    } else if &const_zig[c_name] != c_val	{log_diff_buff .write(format!("{}\t{}\t{}\n",c_name,c_val,const_zig[c_name]).as_bytes()).unwrap();}
  };
  for (c_name,c_val) in &dupe_zig 	{log_dupe_z_buff .write(format!("{}\t{}\n",c_name,c_val).as_bytes()).unwrap();};
  for (c_name,c_val) in &dupe_this	{log_dupe_th_buff.write(format!("{}\t{}\n",c_name,c_val).as_bytes()).unwrap();};
  log_extra_buff.flush().unwrap();
  log_miss_buff.flush().unwrap();
  log_diff_buff.flush().unwrap();
  log_dupe_z_buff.flush().unwrap();
  log_dupe_th_buff.flush().unwrap();

  match blank_p_ {
    Some(blank_p) => {
      let log_blank_p:&Path      	= Path::new("./data/winConst_vs_ziggle_blank.log");
      // if log_blank_p.is_file()	{return Err(format!("Aborting, file exists {:?}",log_blank_p).into())};
      let log_blank_f            	= File::create (&log_blank_p).unwrap();
      let mut log_blank_buff     	= BufWriter::new(log_blank_f);
      log_blank_buff.write("# Constants present in Ziggle, but blank in this crate\n".as_bytes()).unwrap();
      log_blank_buff.write(&header).unwrap();
      let (const_this_blank,dupe_this_blank)	= get_const_kv_from(blank_p).unwrap();
      for (c_name,c_val) in &const_zig {
        if   !const_this      .contains_key(c_name)	{
          if !const_this_blank.contains_key(c_name)	{log_miss_buff .write(format!("{}\t{}\n"    ,c_name,c_val).as_bytes()).unwrap();
          } else                                   	{log_blank_buff.write(format!("{}\t{}\n"    ,c_name,c_val).as_bytes()).unwrap();};};};
      log_blank_buff.flush().unwrap();
    },
    None	=> {}}
}

fn merge_this_with_ziggle() {
  // let ziggle_p       	:&Path	= Path::new("../winAPIconst/data/ziggle_clean64.txt");
  // let this_p         	:&Path	= Path::new("./data/winConst_Valid.txt");
  // let merged_p       	:&Path	= Path::new("./data/winConst_Valid_ziggle.txt");
  // let merged_f       	= File::create(&merged_p).unwrap();
  // let mut merged_buff	= BufWriter::new(merged_f);

  // let mut sopts = SearchOpts::default(); sopts.add_option(SearchOpt::ValInd(col_i));
  // let const_zig      	:HashMap<String,String> = get_const_kv_from(ziggle_p,&sopts).unwrap();
  // let const_this     	:HashMap<String,String> = get_const_kv_from(this_p  ,&sopts).unwrap();
  // let mut win32_const	:BTreeMap<String,String>	= BTreeMap::new();
  // for (c_name,c_val) in &const_this {
  //   win32_const.insert(c_name.to_string(),c_val.to_string());};
  // for (c_name,c_val) in &const_zig {
  //   if   !const_this.contains_key(c_name)	{win32_const.insert(c_name.to_string(),c_val.to_string());}};
  // for (c_name,c_val) in &win32_const {
  //   merged_buff.write(format!("{}\t{}\n",c_name,c_val).as_bytes()).unwrap();};
  // merged_buff.flush().unwrap();
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

#[cfg(feature="rustdoc")]
pub mod trustfall_rustdoc;
#[cfg(feature="rustdoc")]
use trustfall_rustdoc::{rustdoc_find_consts,rustdocs_to_tsv};

fn dedupe_const_csv(csv_p:&Path) -> Result<(),Box<dyn Error>>{
  let mut csv_dedupe_p	= PathBuf::from(csv_p); // ./data/winConst_bindgen_All_185k
  let mut log_dupe_p  	= csv_dedupe_p.clone();
  let parent          	= csv_p.parent()   .unwrap();                 	// ./data/
  let stem_in         	= csv_p.file_stem().unwrap_or(OsStr::new(""));	// winConst_bindgen_All_185k
  let ext_in          	= csv_p.extension().unwrap_or(OsStr::new(""));	//
  let stem_out        	= concat_os_str2(&stem_in,&OsStr::new("_dedupe"));
  csv_dedupe_p.set_file_name(&stem_out);
  csv_dedupe_p.set_extension(&ext_in);
  log_dupe_p.set_file_name(&stem_out);
  log_dupe_p.set_extension("log");

  // p!("{:?}",csv_p); p!("{:?}",csv_dedupe_p); p!("{:?}",log_dupe_p);
  // if log_dupe_p.is_file() {return Err(format!("Aborting, file exists {:?}",log_dupe_p).into())};
  // if csv_dedupe_p.is_file() {return Err(format!("Aborting, file exists {:?}",csv_dedupe_p).into())};
  let csv_dedupe_f	= File::create(&csv_dedupe_p).unwrap();	let mut csv_dedupe_buff	= BufWriter::new(csv_dedupe_f);
  let log_dupe_f  	= File::create(&log_dupe_p  ).unwrap();	let mut log_dupe_buff  	= BufWriter::new(log_dupe_f);

  // let header	= "Name\tType\tTypePrimitive\tValue\tNamespace\n".as_bytes();
  log_dupe_buff.write("# DeDuplicated constants\n".as_bytes())?;
  let mut rdr	= csv::ReaderBuilder::new().has_headers(true).delimiter(b'\t').comment(Some(b'#')).from_path(csv_p)?;
  let hd = rdr.byte_headers()?;
  for field in hd.iter() {
    csv_dedupe_buff.write(&field)?; csv_dedupe_buff.write(tab)?;
    log_dupe_buff  .write(&field)?; log_dupe_buff  .write(tab)?;
  }
  csv_dedupe_buff.write(nl)?;
  log_dupe_buff  .write(nl)?;

  let (const_this,dupe_this) = get_const_kvals_from(csv_p).unwrap();
  // c_vals contains the key as well, so no need to insert c_name
  for (c_name,c_vals) in &const_this	{csv_dedupe_buff.write(format!("{}\n",c_vals.join("\t")).as_bytes()).unwrap();};
  for (c_name,c_vals) in &dupe_this 	{log_dupe_buff  .write(format!("{}\n",c_vals.join("\t")).as_bytes()).unwrap();};

  csv_dedupe_buff.flush().unwrap();
  log_dupe_buff  .flush().unwrap();
  Ok(())
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
    #[cfg(feature="rustdoc")]
    p!("converting Windows_sys rustdocs to TSV");
    #[cfg(feature="rustdoc")]
    rustdocs_to_tsv();	// 1 Windows_sys rustdocs → TSV
  } else if let Some(pos) = args.iter().position(|x| *x == "wmd2ziggle") {
    let this_p:&Path	= Path::new("./data/winConst_bindgen_All_185k");
    compare_this_to_ziggle(ziggle_p,this_p,None); // 2 Compares winConst files ↑ to a ziggle database and generates lists of differences (extra constants, missing constants, constants with different values)
  } else if let Some(pos) = args.iter().position(|x| *x == "rdoc2ziggle") {
    let this_p      	:&Path	= Path::new("./data/winConst_Valid.txt");
    let this_blank_p	:&Path	= Path::new("./data/winConst_Blank.txt");
    compare_this_to_ziggle(ziggle_p,this_p,Some(this_blank_p)); // 2 Compares winConst files ↑ to a ziggle database and generates lists of differences (extra constants, missing constants, constants with different values)
  } else if let Some(pos) = args.iter().position(|x| *x == "merge2ziggle") {
    merge_this_with_ziggle();	// 3 Add missing constants from the ziggle database
  } else if let Some(pos) = args.iter().position(|x| *x == "test") {
    // test1()
    // let tsv		= "21\t22\t23".to_string();
    // let result = get_col_val(tsv,SearchOpts::default().add_option(SearchOpt::ValInd(2)));
    // p!("{:?}",result);
  }
}
