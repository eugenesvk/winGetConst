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

use std::fs::File;
use std::io::{Write,BufReader,BufRead,Error};
pub fn rustdoc_find_consts_adapter_directly(crate_rustdoc_path:&Path,query_path:&Path) -> Result<(),Error> {
  // fails with the wrong version of rustdocs (mismatching trustfall-rustdoc-adapter), so use trustfall_rustdoc that deals with versions wrapping various adapter: e.g., using v27 on v26 crate docs Error("unknown variant `typedef`, expected one of `module extern_crate import struct struct_field union enum variant function type_alias opaque_ty constant trait trait_alias impl static foreign_type macro proc_attribute proc_derive assoc_const assoc_type primitive keyword`", line: 19138, column: 29)'
  use trustfall_rustdoc_adapter::{Crate,IndexedCrate,RustdocAdapter};
  let content = std::fs::read_to_string(crate_rustdoc_path).with_context(|| format!("✗load {:?} file",crate_rustdoc_path)).expect("✗load rustdoc");

  let crate_root	:Crate       	= serde_json::from_str(&content).expect("failed to parse rustdoc");  // root of the emitted JSON blob, contains all type/documentation information about the language items in the local crate, as well as info about external items to allow tools to find or link to them
  let crate_rdoc	:IndexedCrate	= IndexedCrate::new(&crate_root); // rustdoc for a crate, together with associated indexed data to speed up common operations
  let adapter   	= Arc::new(RustdocAdapter::new(&crate_rdoc, None));

  let query_const	= std::fs::read_to_string(query_path).with_context(|| format!("✗load {:?} file",query_path)).expect("✗load query");
  let query_const = r#"{Crate {item {
    ... on Constant {
                       id @output
                       name @output
                       expr @output
                       value @output
                       #type_ @output
                       #type: type_ @output # works with r#type
                       is_literal @output
      importable_path {path @output}
    }}}}"#;
  let query_assoc = r#"{Crate {item {
    ... on Trait {
      trait_name:        name @output
      associated_constant {
                         id @output
                         name @output
                         #type_ @output
                         default @output
    }}}}}"#;
  let query	= query_const;
  let variables: BTreeMap<&str, &str> = BTreeMap::default();
  // let schema = Schema::parse(include_str!("../test_data/rustdoc_schema.graphql")).expect("✗parse schema");
  let schema = Schema::parse(include_str!("D:\\Dev\\repo\\Rust\\trustfall-rustdoc-adapter\\src\\rustdoc_schema.graphql")).expect("✗parse schema");

  #[derive(Debug,PartialOrd,Ord,PartialEq,Eq,serde::Deserialize,serde::Serialize)]
  // #[derive(Debug,PartialEq,Eq,serde::Deserialize,serde::Serialize)]
  struct OutAssocConst {
    id      	: String,
    name    	: String,
    //      	AssocConstant field type should match https://docs.rs/rustdoc-types/latest/rustdoc_types/enum.ItemEnum.html#variant.AssocConst
    // type_	: String,
    default 	: Option<String>,
  }
  #[derive(Debug,PartialOrd,Ord,PartialEq,Eq,serde::Deserialize,serde::Serialize)]
  struct OutConst {
    id        	: String,
    name      	: String,
    path      	: Vec   <String>,
    //        	Constant field type should match https://docs.rs/rustdoc-types/latest/rustdoc_types/struct.Constant.html
    // type_  	: String,
    // r#type 	: String, // works with `type: type_ @output`
    // type_  	: rustdoc_types::Type, // Type→FieldValue not implemented, use ↓ json→string
    expr      	: String,
    value     	: Option<String>,
    is_literal	: bool,
  }
  type OutType = OutConst;
  let results:Vec<_> = trustfall::execute_query(&schema, adapter.clone(), query, variables.clone()).expect("failed to run query")
    .map(|row| row.try_into_struct::<OutType>().expect("shape mismatch")).collect();
  // results.sort_unstable();
  // p!("\n{}", serde_json::to_string_pretty(&results).unwrap());

  use rustdoc_types::{Id,Item,ItemEnum};
  use std::{collections::HashMap};
  let crate_index:HashMap<Id,Item> = crate_root.index;
  p!("name \t val \t type \t value_orig \t expr \t is_literal \t type_orig");


  // 1 how to extract the GUID from this type?
  // pub const SWbemNamedValue: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x04b83d60_21ae_11d2_8b33_00600806d9b6);
  // 2 why does ziggle has a prefix?
  // ziggle: CLSID_SWbemNamedValue	{04b83d60-21ae-11d2-8b33-00600806d9b6}



  use std::collections::BTreeMap;
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

    // if i > 500 { break }; // use on huge Windows crate
    use rustdoc_types::Type;
    let val   	:String	= rustdoc_const_value(&c_value,&c_expr,&out_type);
    let c_name	:String	= out.name.to_string();
    win_api_constants.insert(c_name, (val,out_type_short.to_string()));
    // write!(output, "{}\t{}\t{}\n",out.name,val,out_type_short)?;
    // p!("{:?}\t= {:?} \t= {:?} \t= {:?} \t= {:?} \t {:?} \t {:?}"
      // ,out.name,val,out_type_short,c_value,c_expr,out.is_literal,"out_type");
    };
  for (i,(c_name,c_val_type)) in win_api_constants.iter().enumerate() {
    // if i > 500 { break }; // use on huge Windows crate
    let c_val 	= &c_val_type.0;
    let c_type	= &c_val_type.1;
    write!(out_all, "{}\t{}\t{}\n",c_name,c_val,c_type)?;
    if c_val=="_"	{write!(out_blank, "{}\t{}\t{}\n",c_name,c_val,c_type)?;
    } else       	{write!(out_valid, "{}\t{}\t{}\n",c_name,c_val,c_type)?;}
    // p!("{:?}\t= {:?} \t= {:?}",c_name,c_val,c_type)
    };




    // if out.expr != "_" {
    //   v = out.expr.clone();
    // } else {
    //   v = match out.value.clone() {
    //     Some(v)	=> v,
    //     None   	=> "_".to_string(),
    //   };
    // };
    // let out_type = match serde_json::from_str::<rustdoc_types::Type>(&out.type_).unwrap() {
      // rustdoc_types::Type::Primitive(t)         	=> t,
      // rustdoc_types::Type::BorrowedRef{type_,..}	=> type_,
      // _                                         	=> "✗other type"&,
      //     "type"                                	:{"borrowed_ref":{"lifetime": "'static","mutable": false,"type":{"primitive": "str"}}},
      //     "type"                                	:{"resolved_path":{"name":"Years","id": "0:3:1633","args":{"angle_bracketed":{"args":[],"bindings":[]}}}},
      // Err(e)                                    	=> rustdoc_types::Type::Primitive("✗".to_string()),
    // };
    // let out_type = match serde_json::from_str::<rustdoc_types::Type>(&out.type_) {
    //   Ok(t) 	=> t,
    //   Err(e)	=> rustdoc_types::Type::Primitive("✗".to_string()),

  // get rustdoc_types::Type enum back from json string
  // for out in &results {p!("{:?}\t= {:?}",out.type_,serde_json::from_str::<rustdoc_types::Type>(&out.type_),);}

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
  Ok(())
}