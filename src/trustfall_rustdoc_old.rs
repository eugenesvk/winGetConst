use anyhow::Context;
use std::collections::BTreeMap;
use std::path::Path;
use std::sync::Arc;
use trustfall::{Schema, TryIntoStruct};

pub fn rustdoc_find_consts_adapter_directly(crate_rustdoc_path:&Path,query_path:&Path) {
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