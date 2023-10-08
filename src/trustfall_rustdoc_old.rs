#![allow(unused_imports,unused_variables,unreachable_code,dead_code,non_upper_case_globals,non_snake_case)]
extern crate helper;
use std::path::PathBuf;
use helper        	::*; // gets macros
use helper::alias 	::*;
use helper::helper	::*;

use anyhow::Context;
use std::collections::BTreeMap;
use std::path::Path;
use std::sync::Arc;
use trustfall::{Schema, TryIntoStruct};

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
  // fails with the wrong version of rustdocs (mismatching trustfall-rustdoc-adapter), so use trustfall_rustdoc that deals with versions wrapping various adapter: e.g., using v27 on v26 crate docs Error("unknown variant `typedef`, expected one of `module extern_crate import struct struct_field union enum variant function type_alias opaque_ty constant trait trait_alias impl static foreign_type macro proc_attribute proc_derive assoc_const assoc_type primitive keyword`", line: 19138, column: 29)'
  use trustfall_rustdoc_adapter::{Crate,IndexedCrate,RustdocAdapter};
  let crate_rustdoc_path = "./test_data/pub_module_level_const_missing_mod.json";
  let content = std::fs::read_to_string(crate_rustdoc_path)
    .with_context(|| format!("Could not load {:?} file",crate_rustdoc_path)).expect("failed to load rustdoc");

  let crate_root	:Crate       	= serde_json::from_str(&content).expect("failed to parse rustdoc");  // root of the emitted JSON blob, contains all type/documentation information about the language items in the local crate, as well as info about external items to allow tools to find or link to them
  let crate_rdoc	:IndexedCrate	= IndexedCrate::new(&crate_root); // rustdoc for a crate, together with associated indexed data to speed up common operations
  let adapter   	= Arc::new(RustdocAdapter::new(&crate_rdoc, None));

  let query	= std::fs::read_to_string(query_path)
    .with_context(|| format!("Could not load {:?} file",query_path)).expect("failed to load query");
  let query = r#"{Crate {item {
    ... on Constant {
                       name @output
                       expr @output
                       value @output
                       type_ @output
                       #type: type_ @output # works with r#type
                       is_literal @output
      importable_path {path @output}
    }}}}"#;
  let variables: BTreeMap<&str, &str> = BTreeMap::default();

  // let schema = Schema::parse(include_str!("../test_data/rustdoc_schema.graphql")).expect("schema failed to parse");
  let schema = Schema::parse(include_str!("D:\\Dev\\repo\\Rust\\trustfall-rustdoc-adapter\\src\\rustdoc_schema.graphql")).expect("schema failed to parse");

  // #[derive(Debug,PartialOrd,Ord,PartialEq,Eq,serde::Deserialize,serde::Serialize)]
  #[derive(Debug,PartialEq,Eq,serde::Deserialize,serde::Serialize)]
  struct Output {
    name      	: String,
    value     	: Option<String>,
    expr      	: String,
    // type_  	: rustdoc_types::Type, // Type→FieldValue not implemented, use ↓ json→string
    // r#type 	: String, // works with `type: type_ @output`
    type_     	: String,
    path      	: Vec   <String>,
    is_literal	: bool,
  }
  let results:Vec<_> = trustfall::execute_query(&schema, adapter.clone(), query, variables.clone()).expect("failed to run query")
    .map(|row| row.try_into_struct::<Output>().expect("shape mismatch")).collect();
  // results.sort_unstable();
  println!("\n{}", serde_json::to_string_pretty(&results).unwrap());

  // println!("name \t value \t expr \t is_literal \t@ path");
  // for out in &results {println!("{:?}\t= {:?} \t= {:?} \t= {:?} \t@ {:?}",out.name,out.value,out.expr,out.is_literal,out.path,);}
  // get rustdoc_types::Type enum back from json string
  // for out in &results {println!("{:?}\t= {:?}",out.type_,serde_json::from_str::<rustdoc_types::Type>(&out.type_),);}

  // similar_asserts::assert_eq!(
  //   vec![Output {name:"FIRST" .into(), path:vec!["consts".into(),                "FIRST" .into()],},
  //        Output {name:"SECOND".into(), path:vec!["consts".into(), "inner".into(),"SECOND".into()],
  //     },
  //   ],
  //   results
  // );

  // Ensure that querying for GlobalValue items also retrieves all consts.
  // GlobalValue has no "value" unlike Const, not sure it should be added and whether that has effect on other values
  // let global_values_query = r#"{Crate {
  //   item {
  //     ... on GlobalValue {
  //                        name @output
  //                      value @output
  //       importable_path {path @output}
  //     }}}}"#;
  // let mut global_values_results: Vec<_> =
  //   trustfall::execute_query(&schema, adapter, global_values_query, variables).expect("failed to run query")
  //     .map(|row| row.try_into_struct().expect("shape mismatch"))
  //     .collect();
  // global_values_results.sort_unstable();
  // assert_eq!(results, global_values_results);
}