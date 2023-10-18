use super::*;

pub fn writer(writer: &Writer, def: TypeDef) -> TokenStream {
    if writer.sys	{gen_sys_interface(writer, def)
    } else       	{gen_win_interface(writer, def)}
}

fn gen_sys_interface(writer: &Writer, def: TypeDef) -> TokenStream {
    let name = writer.reader.type_def_name(def);
    let ident = to_ident(name);

    // if type_def_is_exclusive(writer.reader, def)	{quote! {}
    // } else                                      	{quote! {pub type #ident = *mut ::core::ffi::c_void;}}
    quote! {}
}

fn gen_win_interface(writer: &Writer, def: TypeDef) -> TokenStream {
    let generics = &type_def_generics(writer.reader, def);
    let ident = writer.type_def_name(def, generics);
    let is_exclusive = type_def_is_exclusive(writer.reader, def);
    let phantoms = writer.generic_phantoms(generics);
    let constraints = writer.generic_constraints(generics);
    let cfg = type_def_cfg(writer.reader, def, &[]);
    let doc = writer.cfg_doc(&cfg);
    let features = writer.cfg_features(&cfg);
    let interfaces = type_interfaces(writer.reader, &Type::TypeDef(def, generics.to_vec()));
    let vtables = type_def_vtables(writer.reader, def);
    let has_unknown_base = matches!(vtables.first(), Some(Type::IUnknown));

    let mut tokens = quote! { };

    tokens.combine(&writer.interface_trait(def, generics, &ident, &constraints, &features, has_unknown_base));
    tokens
}
