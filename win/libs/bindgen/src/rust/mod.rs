#![allow(unused_variables,dead_code)]
extern crate helper;
#[allow(unused_imports)] use helper        	::*; // gets macros
#[allow(unused_imports)] use helper::alias 	::*;
#[allow(unused_imports)] use helper::helper	::*;
#[allow(unused_imports)] use helper::fs    	::*;
#[allow(unused_imports)] use helper::parser	::*;

mod cfg;
mod com_methods;
mod constants;
mod enums;
mod handles;
mod interfaces;
mod iterators;
mod method_names;
mod standalone;
mod try_format;
mod winrt_methods;
mod writer;
use super::*;
use crate::{Error, Result, Tree};
use cfg::*;
use rayon::prelude::*;

pub fn from_reader(reader: &metadata::Reader, filter: &metadata::Filter, mut config: std::collections::BTreeMap<&str, &str>, output: &str) -> Result<()> {
    let mut writer  	= Writer::new(reader, filter, output);
    writer.package  	= config.remove("package"  ).is_some();
    writer.flatten  	= config.remove("flatten"  ).is_some();
    writer.std      	= config.remove("std"      ).is_some();
    writer.sys      	= config.remove("sys"      ).is_some() || writer.std;
    writer.implement	= config.remove("implement").is_some();
    writer.minimal  	= config.remove("minimal"  ).is_some();

    if writer.package   && writer.flatten           	{return Err(Error::new("cannot combine `package` and `flatten` configuration values"));}
    if writer.implement && writer.sys               	{return Err(Error::new("cannot combine `implement` and `sys` configuration values"));}
    if let Some((key, _)) = config.first_key_value()	{return Err(Error::new(&format!("invalid configuration value `{key}`")));}

    if writer.package	{gen_package(&writer)
    } else           	{gen_file(&writer)}
}

fn gen_file(writer: &Writer) -> Result<()> {
    // TODO: harmonize this output code so we don't need these two wildly differnt code paths
    // there should be a simple way to generate the with or without namespaces.

    if writer.flatten {
        let tokens = standalone::standalone_imp(writer);
        crate::write_to_file(writer.output, try_format(writer, &tokens))
    } else {
        let mut tokens = String::new();
        let root = Tree::new(writer.reader, writer.filter);

        for tree in root.nested.values() {
            tokens.push_str(&namespace(writer, tree));
        }

        crate::write_to_file(writer.output, try_format(writer, &tokens))
    }
}

fn gen_package(writer: &Writer) -> Result<()> {
    let directory = crate::directory(writer.output);
    let root = Tree::new(writer.reader, writer.filter);
    let mut root_len = 0;

    for tree in root.nested.values() {
        root_len = tree.namespace.len();
        _ = std::fs::remove_dir_all(format!("{directory}/{}", tree.namespace));
    }

    let trees = root.flatten();

    trees.par_iter().try_for_each(|tree| {
        let directory = format!("{directory}/{}", tree.namespace.replace('.', "/"));
        let tokens = namespace(writer, tree);

        let tokens_impl = if !writer.sys { namespace_impl(writer, tree) } else { String::new() };

        if !writer.sys && !tokens_impl.is_empty() {
            // tokens.push_str("#[cfg(feature = \"implement\")]\n::core::include!(\"impl.rs\");\n");
        }

        let output = format!("{directory}/mod.rs");
        crate::write_to_file(&output, try_format(writer, &tokens))?;

        if !writer.sys && !tokens_impl.is_empty() {
            let output = format!("{directory}/impl.rs");
            crate::write_to_file(&output, try_format(writer, &tokens_impl))?;
        }

        Ok::<(), Error>(())
    })?;

    let cargo_toml = format!("{}/Cargo.toml", crate::directory(directory));
    let mut toml = String::new();

    for line in crate::read_file_lines(&cargo_toml)? {
        toml.push_str(&line);
        toml.push('\n');

        if line == "# generated features" {
            break;
        }
    }

    for tree in trees.iter().skip(1) {
        let feature = tree.namespace[root_len + 1..].replace('.', "_");

        if let Some(pos) = feature.rfind('_') {
            let dependency = &feature[..pos];

            toml.push_str(&format!("{feature} = [\"{dependency}\"]\n"));
        } else {
            toml.push_str(&format!("{feature} = []\n"));
        }
    }

    crate::write_to_file(&cargo_toml, toml)
}

use crate::tokens::*;
use metadata::*;
use method_names::*;
use std::collections::*;
use std::fmt::Write;
use try_format::*;
use writer::*;

pub const clsid_:&str = "CLSID_";
pub const iid_  :&str = "IID_";
pub const guid0 :&str = "{00000000-0000-0000-0000-000000000000}";
fn namespace(writer: &Writer, tree: &Tree) -> String {
    let writer = &mut writer.clone();
    writer.namespace = tree.namespace;
    let mut tokens = TokenStream::new();

    for (name, tree) in &tree.nested {
        let name = to_ident(name);
        let namespace_feature = tree.namespace[tree.namespace.find('.').unwrap() + 1..].replace('.', "_");
        if writer.package {
            // tokens.combine(&quote! {pub mod #name;});
        } else {
            // tokens.combine(&quote! { pub mod #name });
            // tokens.push_str("{");
            tokens.push_str(&namespace(writer, tree));
            // tokens.push_str("}");
        }
    }

    let mut types = BTreeMap::<TypeKind, BTreeMap<&str, TokenStream>>::new();

    let feature_ns  = writer::to_feature(&writer.namespace);
    for item in writer.reader.namespace_items(writer.namespace, writer.filter) {
        // println!("name = {:?}",&name); //"DWMFLIP3D_DEFAULT"
        match item {
            Item::Type(def) => {
                // "Public¦Sealed¦SequentialLayout¦" "FaxOutgoingJobs" "Windows.Win32.Devices.Fax" "" "" "" "" "" None None "" "" ""
                // "Public¦Abstract¦Interface¦" "IFaxOutgoingJobs" "Windows.Win32.Devices.Fax" "" "" "" "" "" None None "" "" ""
                // NOT CLSID [issue](https://github.com/microsoft/win32metadata/issues/737): `"Public¦Sealed¦SequentialLayout¦" "CONFIGURATION_ENTRY" "Windows.Win32.System.Iis" "" "" "" "" "" None None "" "" ""`
                // let type_def_flags = writer.reader.type_def_flags(def);
                // let mut type_def_flags_str = "".to_string();
                // if type_def_flags.contains(TypeAttributes::Public)          	{type_def_flags_str += "Public¦"};
                // if type_def_flags.contains(TypeAttributes::ExplicitLayout)  	{type_def_flags_str += "ExplicitLayout¦"};
                // if type_def_flags.contains(TypeAttributes::Abstract)        	{type_def_flags_str += "Abstract¦"};
                // if type_def_flags.contains(TypeAttributes::Sealed)          	{type_def_flags_str += "Sealed¦"};
                // if type_def_flags.contains(TypeAttributes::WindowsRuntime)  	{type_def_flags_str += "WindowsRuntime¦"};
                // if type_def_flags.contains(TypeAttributes::Interface)       	{type_def_flags_str += "Interface¦"};
                // if type_def_flags.contains(TypeAttributes::SequentialLayout)	{type_def_flags_str += "SequentialLayout¦"};
                // if type_def_flags.contains(TypeAttributes::Import)          	{type_def_flags_str += "Import¦"};
                // println!("{:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}"
                // ,type_def_flags_str
                // // fn type_def_flags(&self, row: TypeDef) -> TypeAttributes: impl TypeAttributes {Public ExplicitLayout Abstract Sealed WindowsRuntime Interface SequentialLayout Import}
                // ,writer.reader.type_def_name(def)
                // ,writer.reader.type_def_namespace(def)
                // ,"" //writer.reader.type_def_extends(def)
                // ,"" //writer.reader.type_def_methods(def)
                // ,"" //writer.reader.type_def_fields(def)
                // ,"" //writer.reader.type_def_generics(def)
                // ,"" //writer.reader.type_def_interface_impls(def)
                // ,writer.reader.type_def_enclosing_type(def)
                // ,writer.reader.type_def_class_layout(def)

                // ,writer.reader.type_def_kind(def)

                // ,"" //writer.reader.type_ref_name(def)
                // ,"" //writer.reader.type_ref_namespace(def)
                // ,"" //writer.reader.type_ref_resolution_scope(def)
                // );

                let type_name = writer.reader.type_def_type_name(def);
                if REMAP_TYPES.iter().any(|(x, _)| x == &type_name) {
                    continue;
                }
                if CORE_TYPES.iter().any(|(x, _)| x == &type_name) {
                    continue;
                }
                let name = type_name.name;
                let kind = writer.reader.type_def_kind(def);
                match kind {
                    TypeKind::Class => {
                        // if writer.reader.type_def_flags(def).contains(TypeAttributes::WindowsRuntime) {
                            // types.entry(kind).or_default().insert(name, classes::writer(writer, def));
                        // }
                    }
                    TypeKind::Interface => types.entry(kind).or_default().entry(name).or_default().combine(&interfaces::writer(writer, def)),
                    TypeKind::Enum => {}, //types.entry(kind).or_default().entry(name).or_default().combine(&enums::writer(writer, def)),
                    TypeKind::Struct => {
                        if writer.reader.type_def_fields(def).next().is_none() { // CLSID
                            // "The way I handle this is to only treat it as a CLSID if the struct has zero fields." https://github.com/microsoft/win32metadata/issues/737#issuecomment-964404006
                            if let Some(guid) = type_def_guid(writer.reader, def) {
                                let ident = to_ident(name);
                                let value = writer.guid(&guid);
                                let type_ = Type::GUID;
                                let type_nm = writer.type_name(&type_);
                                let type_prim = type_to_primitive(writer.reader, &type_);
                                let constant = quote! {#clsid_  #ident #tab #type_nm #tab #type_prim #tab #value #tab #feature_ns;};
                                types.entry(TypeKind::Class).or_default().entry(name).or_default().combine(&constant);
                                continue;
                            }
                        }
                        // types.entry(kind).or_default().entry(name).or_default().combine(&structs::writer(writer, def));
                    }
                    TypeKind::Delegate => {}, //types.entry(kind).or_default().entry(name).or_default().combine(&delegates::writer(writer, def)),
                }
            }
            Item::Fn(def, namespace) => {
                // let name = writer.reader.method_def_name(def);
                // functions.entry(name).or_default().combine(&functions::writer(writer, &namespace, def));
            }
            Item::Const(def) => {
                let name = writer.reader.field_name(def);
                // println!("name = {:?}",&name); //"DWMFLIP3D_DEFAULT"
                // println!("append = {:?}",&constants::writer(writer, def)); //TokenStream("DWMFLIP3D_DEFAULT¦DWMFLIP3DWINDOWPOLICY¦i32¦0")
                types.entry(TypeKind::Class).or_default().entry(name).or_default().combine(&constants::writer(writer, def));
            }
        }
    }

    for ty in types.values().flat_map(|v| v.values()) {
        tokens.combine(ty);
    }

    if writer.implement {
        tokens.push_str(&namespace_impl(writer, tree));
    }

    tokens.into_string()
}

fn namespace_impl(writer: &Writer, tree: &Tree) -> String {
    let writer = &mut writer.clone();
    writer.namespace = tree.namespace;
    let mut types = BTreeMap::<&str, TokenStream>::new();

    for item in writer.reader.namespace_items(writer.namespace, writer.filter) {
        if let Item::Type(def) = item {
            let type_name = writer.reader.type_def_type_name(def);
            if CORE_TYPES.iter().any(|(x, _)| x == &type_name) {
                continue;
            }
            if writer.reader.type_def_kind(def) != TypeKind::Interface {
                continue;
            }
            // let tokens = implements::writer(writer, def);
            let tokens = quote!{};

            if !tokens.is_empty() {
                types.insert(type_name.name, tokens);
            }
        }
    }

    let types = types.values();

    let tokens = quote! { #(#types)*};

    tokens.into_string()
}

/// Expand a possibly empty generics list with a new generic
fn expand_generics(generics: TokenStream, new: TokenStream) -> TokenStream {
    if generics.is_empty() {
        quote!(#new)
    } else {
        quote!(#generics, #new)
    }
}

/// Expand a possibly emppty where clause with a new generic constraint
fn expand_where_clause(where_clause: TokenStream, generic: TokenStream) -> TokenStream {
    if where_clause.is_empty() {
        quote!(where #generic)
    } else {
        quote!(#where_clause #generic)
    }
}
