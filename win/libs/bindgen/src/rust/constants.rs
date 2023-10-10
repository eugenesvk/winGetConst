use super::*;
extern crate helper;
#[allow(unused_imports)] use helper        	::*; // gets macros
#[allow(unused_imports)] use helper::alias 	::*;
#[allow(unused_imports)] use helper::helper	::*;
#[allow(unused_imports)] use helper::fs    	::*;
#[allow(unused_imports)] use helper::parser	::*;

pub fn writer(writer: &Writer, def: Field) -> TokenStream {
    let name = to_ident(writer.reader.field_name(def));
    let ty = writer.reader.field_type(def, None).to_const_type();
    let cfg = field_cfg(writer.reader, def);
    let doc = writer.cfg_doc(&cfg);
    let features = writer.cfg_features(&cfg);

    // println!("def= {:?} ty={:?}",&def,&ty); //def=Field(Row{row:15644,file:2})ty=TypeDef(TypeDef(Row{row:3837,file:2}),[])
    if let Some(constant) = writer.reader.field_constant(def) {
        let constant_type = writer.reader.constant_type(constant);

        if ty == constant_type {
            if ty == Type::String {
                let crate_name = writer.crate_name();
                if field_is_ansi(writer.reader, def) {
                      	{let value = writer.value(      &writer.reader.constant_value(constant));
                      	quote! {#name #tab PCSTR  #tab str #tab #value;}
                } else	{let value = writer.value(      &writer.reader.constant_value(constant));
                      	quote! {#name #tab PCWSTR #tab str #tab #value;}}
            } else    	{let value_t = writer.typed_value(&writer.reader.constant_value(constant));
                      	quote! {#name #tab #value_t;}}
        } else {
            let kind = writer.type_default_name(&ty);
            let value = writer.value(&writer.reader.constant_value(constant));
            let underlying_type = type_underlying_type(writer.reader, &ty);

            let value = if underlying_type == constant_type {
                                             // value
                                            TokenStream(parse_lit(&value.to_string()))
            } else if writer.std && underlying_type == Type::ISize {
                  	 quote! { ::core::ptr::invalid_mut(#value as _) } // todo: convert to actual value
            } else	{quote! {                          #value as _ }};

            if !writer.sys && type_has_replacement(writer.reader, &ty) {

            if !writer.sys && type_has_replacement(writer.reader, &ty) { //HRESULT|PCSTR|PCWSTR|has_attr(NativeTypedefAttribute)|TypeKind::Enum
                  	let type_prim = type_to_primitive(writer.reader, &ty);
                  	 quote! {#name #tab #kind #tab #type_prim #tab #value;}
            } else	{quote! {#name #tab #kind #tab _          #tab #value;}}
        }
    } else if let Some(guid) = field_guid(writer.reader, def) {
        let value = writer.guid(&guid);
        let guid = writer.type_name(&Type::GUID);
        let guid = TokenStream("GUID".to_string());
        quote! {#name #tab #guid #tab str #tab #value;} // todo get actual guid value
    } else if let Some(value) = initializer(writer, def) {
        let kind = writer.type_default_name(&ty);
        quote! {#name #tab #kind #tab _ { #value};} // todo get primivite
    } else {quote! {}}
}

fn initializer(writer: &Writer, def: Field) -> Option<TokenStream> {
    let Some(value) = constant(writer, def) else {
        return None;
    };

    let mut input = value.as_str();

    let Type::TypeDef(def, _) = writer.reader.field_type(def, None) else {
        unimplemented!();
    };

    let mut result = quote! {};

    for field in writer.reader.type_def_fields(def) {
        let (value, rest) = field_initializer(writer, field, input);
        input = rest;
        result.combine(&value);
    }

    Some(result)
}

fn field_initializer<'a>(writer: &Writer, field: Field, input: &'a str) -> (TokenStream, &'a str) {
    let name = to_ident(writer.reader.field_name(field));

    match writer.reader.field_type(field, None) {
        Type::GUID => {
            let (literals, rest) = read_literal_array(input, 11);
            let value = writer.guid(&GUID::from_string_args(&literals));
            (quote! { #name: #value, }, rest)
        }
        Type::Win32Array(_, len) => {
            let (literals, rest) = read_literal_array(input, len);
            let literals = literals.iter().map(|literal| TokenStream::from(*literal));
            (quote! { #name: [#(#literals,)*], }, rest)
        }
        _ => {
            let (literal, rest) = read_literal(input);
            let literal: TokenStream = literal.into();
            (quote! { #name: #literal, }, rest)
        }
    }
}

fn constant(writer: &Writer, def: Field) -> Option<String> {
    writer.reader.find_attribute(def, "ConstantAttribute").map(|attribute| {
        let args = writer.reader.attribute_args(attribute);
        match &args[0].1 {
            Value::String(value) => value.clone(),
            rest => unimplemented!("{rest:?}"),
        }
    })
}

fn read_literal(input: &str) -> (&str, &str) {
    let mut start = None;
    let mut end = 0;

    for (pos, c) in input.bytes().enumerate() {
        if start.is_none() {
            if c != b' ' && c != b',' {
                start = Some(pos);
            }
        } else if c == b' ' || c == b',' || c == b'}' {
            break;
        }
        end += 1;
    }

    let Some(start) = start else {
        unimplemented!();
    };

    (&input[start..end], &input[end..])
}

fn read_token(input: &str, token: u8) -> &str {
    for (pos, c) in input.bytes().enumerate() {
        if c == token {
            return &input[pos + 1..];
        } else if c != b' ' && c != b',' {
            break;
        }
    }

    panic!("`{}` expected", token.escape_ascii());
}

fn read_literal_array(input: &str, len: usize) -> (Vec<&str>, &str) {
    let mut input = read_token(input, b'{');
    let mut result = vec![];

    for _ in 0..len {
        let (literal, rest) = read_literal(input);
        result.push(literal);
        input = rest;
    }

    (result, read_token(input, b'}'))
}
