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

/*
use cargo_semver_checks::RequiredSemverUpdate;
lazy_static::lazy_static! {
  static ref TEST_CRATE_NAMES: Vec<String> = get_test_crate_names();
  /// Mapping test crate (pair) name -> (old rustdoc, new rustdoc).
  static ref TEST_CRATE_RUSTDOCS: BTreeMap<String, (VersionedCrate, VersionedCrate)> =
    get_test_crate_rustdocs();
}
/// A query that can be executed on a pair of rustdoc output files, returning instances of a particular kind of semver violation.
#[non_exhaustive] #[derive(Debug, Clone, Serialize, Deserialize)] pub struct SemverQuery {
                   	pub       	id                 	:        String,
                   	pub(crate)	human_readable_name	:        String,
                   	pub       	description        	:        String,
                   	pub       	required_update    	: RequiredSemverUpdate,
  #[serde(default)]	pub       	reference          	: Option<String>,
  #[serde(default)]	pub       	reference_link     	: Option<String>,
                   	pub(crate)	query              	:        String,
  #[serde(default)]	pub(crate)	arguments          	: BTreeMap<String, TransparentValue>,
  /// The top-level error describing the semver violation that was detected. Even if multiple instances of this semver issue are found, this error message is displayed only at most once.
  pub(crate) error_message: String,
  /// Optional template that can be combined with each query output to produce a human-readable description of the specific semver violation that was discovered.
  #[serde(default)]  pub(crate) per_result_error_template: Option<String>,
}
impl SemverQuery {
  pub fn all_queries() -> BTreeMap<String, SemverQuery> {
    let mut queries = BTreeMap::default();
    for query_text in get_query_text_contents() {
      let query: SemverQuery = ron::from_str(query_text).unwrap_or_else(|e| {
        panic!("Failed to parse a query: {e}```ron{query_text}```");});
      let id_conflict = queries.insert(query.id.clone(), query);
      assert!(id_conflict.is_none(), "{id_conflict:?}");
    }
    queries
  }
}

fn get_test_crate_names() -> Vec<String> {
  std::fs::read_dir("./test_crates/")
    .expect("directory test_crates/ not found")
    .map(|dir_entry| dir_entry.expect("failed to list test_crates/"))
    .filter(|dir_entry| { // Only return directories inside `test_crates/` that contain an `old/Cargo.toml` file. This works around finicky git + cargo behavior:
      // - Create a git branch, commit a new test case, and generate its rustdoc.
      // - Cargo will then create `Cargo.lock` files for the crate, which are ignored by git
      // - Check out another branch, and git won't delete the `Cargo.lock` files since they aren't tracked. But we don't want to run tests on those crates!
      if !dir_entry.metadata().expect("failed to retrieve test_crates/ * metadata")
        .is_dir() { return false;}
      let mut test_crate_cargo_toml = dir_entry.path();
      test_crate_cargo_toml.extend(["old", "Cargo.toml"]);
      test_crate_cargo_toml.as_path().is_file()
    })
    .map(|dir_entry| {String::from(String::from(
      dir_entry.path().to_str().expect("failed to convert dir_entry to String"),)
      .strip_prefix("./test_crates/")
      .expect("the dir_entry doesn't start with './test_crates/', which is unexpected",),)})
    .collect()
}

fn load_pregenerated_rustdoc(crate_pair: &str, crate_version: &str) -> VersionedCrate {
  let path = format!("./localdata/test_data/{crate_pair}/{crate_version}/rustdoc.json");
  load_rustdoc(Path::new(&path))
    .with_context(|| format!("Could not load {path} file, did you forget to run ./scripts/regenerate_test_rustdocs.sh ?"))
    .expect("failed to load baseline rustdoc")
}

fn get_test_crate_rustdocs() -> BTreeMap<String, (VersionedCrate, VersionedCrate)> {
  TEST_CRATE_NAMES.iter().map(|crate_pair| {
    let old_rustdoc = load_pregenerated_rustdoc(crate_pair.as_str()	, "old");
    let new_rustdoc = load_pregenerated_rustdoc(crate_pair         	,  "new");
    (crate_pair.clone(), (old_rustdoc, new_rustdoc))
  }).collect()
}


// use cargo_semver_checks::{GlobalConfig, PackageSelection, ReleaseType, Rustdoc, ScopeSelection,};
// fn check_crate() -> anyhow::Result<()> {
//   let old = "../semver-check/cargo-semver-checks-main/test_crates/pub_module_level_const_missing/old";
//   let mut old_pb = PathBuf::new();
//   old_pb.push(old);
//   let baseline_root: Option<PathBuf> = Some(old_pb);
//   let root = baseline_root.unwrap();

//   // let custom_baseline = Some(Rustdoc::from_git_revision(root, baseline_rev));
//   // if let Some(baseline) = custom_baseline { check.with_baseline(baseline); }

//   let queries = SemverQuery::all_queries(); // gets macro-generated query-functions that read query .ron files
//   // p!("{:?}",queries); //{"pub_module_level_const_missing": Se...
//   for query in queries.values() {
//     p!("{}",query.reference.as_deref().unwrap_or(query.description.as_str()));
//   }
//   let check_release = CheckRelease::new() ; //parsed by clap
//   let check: cargo_semver_checks::Check = check_release.into();
//     // CheckRelease { manifest: Manifest { manifest_path: None }
//     // , workspace: Workspace { package: [], workspace: false, all: false, exclude: [] }
//     // , current_rustdoc: None, baseline_version: None, baseline_rev: None, baseline_rustdoc: None, release_type: None, default_features: false
//     // , only_explicit_features: false, features: [], baseline_features: [], current_features: [], all_features: false, verbosity: Verbosity { verbose: 0 , quiet: 0
//     // , baseline_root: Some("../semver-check\\cargo-semver-checks-main\\test_crates\\pub_module_level_const_missing\\old")
//     // , phantom: PhantomData<clap_verbosity_flag::InfoLevel> } }


//   Ok(())





//   // let new = "../semver-check/cargo-semver-checks-main/test_crates/pub_module_level_const_missing/new";

//   // let query_name = "pub_module_level_const_missing";
//   // let query_text = std::fs::read_to_string(format!("./src/lints/{query_name}.ron")).unwrap();
//   // let semver_query: SemverQuery = ron::from_str(&query_text).unwrap();
//   // let crate_pair_name = "hello".to_string();

//   // let (crate_old, crate_new) = &TEST_CRATE_RUSTDOCS[&crate_pair_name];
//   // p!("{:?}{:?}",crate_old,crate_new);
//   // let indexed_crate_old = VersionedIndexedCrate::new(crate_old);
//   // let indexed_crate_new = VersionedIndexedCrate::new(crate_new);
//   // run_query_on_crate_pair(&semver_query,&crate_pair_name,&indexed_crate_new,&indexed_crate_old,);
// }
// cargo rustdoc -- -Zunstable-options --document-private-items --document-hidden-items --cap-lints "warn" --output-format=json
// cargo rustdoc -- --document-private-items --cap-lints "warn" --output-format=json

// fn run_query_on_crate_pair(semver_query:&SemverQuery, crate_pair_name:&String,
//   indexed_crate_new: &VersionedIndexedCrate,
//   indexed_crate_old: &VersionedIndexedCrate,) -> (String, Vec<BTreeMap<String, FieldValue>>) {
//   let adapter = VersionedRustdocAdapter::new(indexed_crate_new,Some(indexed_crate_old)).expect("could not create adapter");
//   let results_iter = adapter.run_query(&semver_query.query, semver_query.arguments.clone()).unwrap();
//   (format!("./test_crates/{crate_pair_name}/"),
//     results_iter
//       .map(|res| res.into_iter().map(|(k, v)| (k.to_string(), v)).collect())
//       .collect::<Vec<BTreeMap<_, _>>>(),
//   )
// }






macro_rules! add_lints {
  ($($name:ident,)*) => {
    #[cfg(test)]
    mod tests_lints {$(#[test]fn $name() {super::tests::check_query_execution(stringify!($name))})*}
    fn get_query_text_contents() -> Vec<&'static str> {
      vec![$(include_str!(concat!("query/", stringify!($name), ".ron")),)*]
    }
  }
}

add_lints!(pub_module_level_const_missing,);
*/



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
