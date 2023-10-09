#![allow(unused_imports,unused_variables,unreachable_code,dead_code,non_upper_case_globals,non_snake_case)]
extern crate helper;
use std::path::PathBuf;
use helper        	::*; // gets macros
use helper::alias 	::*;
use helper::helper	::*;

use anyhow::Context;
use trustfall::{Schema, TryIntoStruct};
use std      	::{env,
  path       	::Path,
  sync       	::Arc,
  fs         	::File,
  io         	::{Write,BufReader,BufRead,Error},
  collections	::{HashMap,BTreeMap},
  iter       	::Peekable,
  time       	::Duration};

pub fn rustdoc_type_as_str(type_:&rustdoc_types::Type) -> String {
  use rustdoc_types::Type::*;
  match type_ {
    ResolvedPath(p)        	=> p.name.to_string(), // {"resolved_path":{"name":"Years","id": "0:3:1633","args":{"angle_bracketed":{"args":[],"bindings":[]}}}},
    // DynTrait(DynTrait)  	=>,
    Generic(s)             	=> s.to_string(),
    Primitive(s)           	=> s.to_string(),
    // FunctionPointer(fp) 	, //Box<FunctionPointer>
    // Tuple(vT)           	=>, //Vec<Type>
    Slice(type_)           	=> rustdoc_type_as_str(&type_),
    Array{type_,..}        	=> rustdoc_type_as_str(&type_),
    // ImplTrait(vGB)      	=>, //Vec<GenericBound>
    // Infer               	=>,
    RawPointer   {type_,..}	=> rustdoc_type_as_str(&type_),
    BorrowedRef  {type_,..}	=> rustdoc_type_as_str(&type_), //{"borrowed_ref":{"lifetime": "'static","mutable": false,"type":{"primitive":"str"}}},
    QualifiedPath{name ,..}	=> name.to_string(),
    _                      	=> serde_json::to_string(&type_).unwrap(),
  }
}
pub fn rustdoc_const_value(val:&Option<String>,expr:&String,type_:&rustdoc_types::Type) -> String {
  //               	       	            	// isLit	Value    	Expr       	Type
  // const MIN     	: usize	= 16       ;	// true 	"16usize"	16         	{"primitive":"usize"}
  // const MIN_SIZE	: usize	= MIN      ;	// false	"16usize"	"MIN"      	{"primitive":"usize"}
  // const LOG_AS  	: &str 	= "batch"  ;	// true 	None     	"\"batch\""	{"borrowed_ref":{"lifetime":null,"mutable":false,"type":{"primitive":"str"}}}
  // const YEAR    	: Years	= Years(42);	// false	None     	"_"        	{"resolved_path":{"name":"Years","id":"0:3:1633","args":{"angle_bracketed":{"args":[],"bindings":[]}}}}
  // const ALS_C   	: ALS_T	= 12i32;    	// true 	"12i32"  	"12i32"    	ResolvedPath(Path{name:"ALS_T",id:Id("0:3:1661"),args:Some(AngleBracketed{args:[],bindings:[]})})
  // const EXPR_2_2	: i32  	= 2 + 2    ;	// false	"4i32"   	"_"        	{"primitive":"i32"}
  // const FN_FIVE 	: i32  	= five()   ;	// false	"5i32"   	"_"        	{"primitive":"i32"}
  // const fn five() -> i32 { 5 };
  // struct Years(i32);
  // pub type ALS_T = i32;
  use rustdoc_types::Type::*;
  let     nonum	= "✗".to_string();
  match type_ {
    Primitive(s)              	=> match val {Some(v) => crate::parse_lit(&v), None => expr.to_string()}, // built in numeric (i*, u*, f*) types, bool, and char
    // Primitive(s)           	=> match val {Some(v) => {p!("primt {:?} {:?} {:?}",&val,&expr,&type_); crate::parse_lit(&v)}, None => expr.to_string()}, // built in numeric (i*, u*, f*) types, bool, and char
    BorrowedRef  {type_,..}   	=> rustdoc_const_value(&Some(expr.to_string()),&expr,&type_), //{"borrowed_ref":{"lifetime": "'static","mutable": false,"type":{"primitive":"str"}}},
    RawPointer   {type_,..}   	=> rustdoc_const_value(&Some(expr.to_string()),&expr,&type_),
    ResolvedPath(p)           	=> match val {Some(v) => crate::parse_lit(&v), None => expr.to_string()}, // {"resolved_path":{"name":"Years","id": "0:3:1633","args":{"angle_bracketed":{"args":[],"bindings":[]}}}},
    // ResolvedPath(p)        	=> match val {Some(v) => {p!("respt {:?} {:?} {:?}",&val,&expr,&type_); crate::parse_lit(&v)}, None => expr.to_string()}, // {"resolved_path":{"name":"Years","id": "0:3:1633","args":{"angle_bracketed":{"args":[],"bindings":[]}}}},
    QualifiedPath{name ,..}   	=> match val {Some(v) => crate::parse_lit(&v), None => expr.to_string()}, //??? todo
    // QualifiedPath{name ,..}	=> match val {Some(v) => {p!("qualp {:?} {:?} {:?}",&val,&expr,&type_); crate::parse_lit(&v)}, None => expr.to_string()}, //??? todo
    Generic(s)                	=> s.to_string(), //Parameterized types
    // Slice(type_)           	=> rustdoc_const_value(&Some(expr.to_string()),&expr,&type_), //???todo
    // Array{type_,..}        	=> rustdoc_const_value(&Some(expr.to_string()),&expr,&type_), //???todo
    // DynTrait(DynTrait)     	=>,
    // FunctionPointer(fp)    	, //Box<FunctionPointer>
    // Tuple(vT)              	=>, //Vec<Type>
    // ImplTrait(vGB)         	=>, //Vec<GenericBound>
    // Infer                  	=>,
    _                         	=> serde_json::to_string(&type_).unwrap(),
  }
}

use trustfall::{FieldValue,TransparentValue};
use trustfall_rustdoc::{load_rustdoc,VersionedCrate,VersionedIndexedCrate,VersionedRustdocAdapter,};

use serde::{Deserialize, Serialize};
/// A query that can be executed rustdoc output file
#[non_exhaustive] #[derive(Debug,Clone,Serialize,Deserialize)] pub struct SemverQuery {
                    pub       	id                       	: String,
                    pub(crate)	human_readable_name      	: String,
                    pub       	description              	: String,
  #[serde(default)] pub       	reference                	: Option<String>,
  #[serde(default)] pub       	reference_link           	: Option<String>,
                    pub(crate)	query                    	: String,
  #[serde(default)] pub(crate)	arguments                	: BTreeMap<String,TransparentValue>,
  /// Top-level error         	                         	describing the semver violation that was detected. Even if multiple instances of this semver issue are found, this error message is displayed only at most once.
                    pub(crate)	error_message            	: String,
  /// Optional template       	                         	that can be combined with each query output to produce a human-readable description of the specific semver violation that was discovered.
  #[serde(default)] pub(crate)	per_result_error_template	: Option<String>,
}
/// A constant with its data
#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,serde::Deserialize,serde::Serialize)] struct OutConst {
  id         	: String,
  name       	: String,
  path_import	: Vec   <String>,
  //         	Constant field type should match https://docs.rs/rustdoc-types/latest/rustdoc_types/struct.Constant.html
  // type_   	: String,
  // r#type  	: String, // works with `type: type_ @output`
  // type_   	: rustdoc_types::Type, // Type→FieldValue not implemented, use ↓ json→string
  expr       	: String,
  value      	: Option<String>,
  is_literal 	: bool,
}

pub fn rustdoc_find_consts(crate_rustdoc_path:&Path,query_path:&Path) -> anyhow::Result<()> {
  let crate_root_v	:VersionedCrate       	= trustfall_rustdoc::load_rustdoc(&crate_rustdoc_path)?; // root of the emitted JSON blob, contains all type/documentation information about the language items in the local crate, as well as info about external items to allow tools to find or link to them
  let crate_rdoc  	:VersionedIndexedCrate	= VersionedIndexedCrate::new(&crate_root_v); // rustdoc for a crate, together with associated indexed data to speed up common operations
  let crate_rdoc_v	                      	= crate_rdoc.version();
  let crate_base  	                      	= None;
  let adapter     	                      	= VersionedRustdocAdapter::new(&crate_rdoc,crate_base)?;
  p!("crate rustdocs version {:?}",crate_rdoc_v);

  let query_text  	            	= std::fs::read_to_string(query_path).with_context(|| format!("✗load {:?} file",query_path)).expect("✗load query");
  let semver_query	:SemverQuery	= ron::from_str(&query_text).unwrap_or_else(|e| {panic!("✗parse a query: {e}```ron{query_text}```");});

  let mut results_iter	= adapter.run_query(&semver_query.query, semver_query.arguments.clone())?.peekable();
  let peeked          	= results_iter.peek(); // ref to next() value without advancing iterator

  let results:Vec<_> = results_iter.map(|row| row.try_into_struct::<OutConst>().expect("shape mismatch")).collect();
  // results.sort_unstable();
  // p!("\n{}", serde_json::to_string_pretty(&results).unwrap());

  // alternative way to see results
  // p!("{:?}",peeked);
  // for out in results_iter {
  //   // default `FieldValue` JSON repr is explicit about its type, so we can get reliable round-trip serialization of types tricky in JSON like integers and floats.
  //   // `TransparentValue` type is like `FieldValue` minus the explicit type representation, so it's more like what we'd expect to normally find in JSON
  //   // for (k,v) in out {
  //     // println!("{} : {:?}",k,v_transp); // 1
  //     // let v_transp:TransparentValue = v.into(); // 2 convert value
  //     // println!("{} : {}",k,serde_json::to_string_pretty(&v_transp).unwrap());
  //   // }
  //   // 3 convert all
  //   let transparent:BTreeMap<_,TransparentValue> = out.into_iter().map(|(k,v)| (k,v.into())).collect();
  //   println!("\n{}", serde_json::to_string_pretty(&transparent).unwrap());
  // }


  // todo: VersionedCrate doesn't provide for a good way to get an index HashMap, so rustdoc-types doesn't seem to help in anyway, can only run loops for a distinct versioned type of a Crate, and rustdoc_types version should match the one needed for e.g. VersionedCrate::V26
  use rustdoc_types::{Id,Item,ItemEnum};
  let trustfall_rustdoc::VersionedCrate::V26(crate_root) = crate_root_v else { todo!() };
  let crate_index:HashMap<Id,Item> = crate_root.index;
  p!("name \t val \t type \t value_orig \t expr \t is_literal \t type_orig");

  let mut win_api_constants:BTreeMap<String,(String,String)> = BTreeMap::new();

  let     out_all_p  	= "winConst_All.txt";
  let     out_valid_p	= "winConst_Valid.txt";
  let     out_blank_p	= "winConst_Blank.txt";
  let mut out_all    	= File::create(out_all_p)?;
  let mut out_valid  	= File::create(out_valid_p)?;
  let mut out_blank  	= File::create(out_blank_p)?;
  for (i,out) in results.iter().enumerate() { // for out in &results {
    // get Constant by Id from Crate to get a Type since Type wasn't added to rustdoc adapter
    let item                 	:&Item               	= &crate_index[&Id(out.id.clone())];
    let item_inner           	:&ItemEnum           	= &item.inner;
    let ItemEnum::Constant(c)	                     	= &item_inner else {unreachable!("expected to have a Constant")};
    let out_type             	:&rustdoc_types::Type	= &c.type_;
    let out_type_s           	:String              	= rustdoc_type_as_str(&out_type);
    let out_type_short       	:&str                	= out_type_s.split("::").last().unwrap();
    let c_value              	:&Option<String>     	= &out.value; // value/expr are availble directly
    let c_expr               	:&       String      	= &out.expr; // see rustdoc_const_value from details on value/expr
    // let out_type          	:&rustdoc_types::Type	= serde_json::from_str(&out.type_).unwrap();
    // let out_type_s        	                     	= serde_json::to_string(&out_type).unwrap();
    // p!("item = {:?}",item); // docs.rs/rustdoc-types/latest/rustdoc_types/struct.Item.html
    // p!("Tshort = {} \t out_type = {:?} \t asstr {:?}",out_type_short, out_type,out_type_s);

    // if i > 500 { break }; // use on huge Windows crate for testing
    use rustdoc_types::Type;
    let val   	:String	= rustdoc_const_value(&c_value,&c_expr,&out_type);
    let c_name	:String	= out.name.to_string();
    win_api_constants.insert(c_name, (val,out_type_short.to_string()));
    // write!(output, "{}\t{}\t{}\n",out.name,val,out_type_short)?;
    // p!("{:?}\t= {:?} \t= {:?} \t= {:?} \t= {:?} \t {:?} \t {:?}"
      // ,out.name,val,out_type_short,c_value,c_expr,out.is_literal,"out_type");
    };
  for (i,(c_name,c_val_type)) in win_api_constants.iter().enumerate() {
    // if i > 500 { break }; // use on huge Windows crate for testing
    let c_val 	= &c_val_type.0;
    let c_type	= &c_val_type.1;
    write!(out_all, "{}\t{}\t{}\n",c_name,c_val,c_type)?;
    if c_val=="_"	{write!(out_blank, "{}\t{}\t{}\n",c_name,c_val,c_type)?;
    } else       	{write!(out_valid, "{}\t{}\t{}\n",c_name,c_val,c_type)?;}
    // p!("{:?}\t= {:?} \t= {:?}",c_name,c_val,c_type)
    };

  Ok(())
}
