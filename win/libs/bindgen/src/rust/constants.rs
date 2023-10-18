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
                if field_is_ansi(writer.reader, def)
                      	{let value = writer.value(        &writer.reader.constant_value(constant));
                      	quote! {#name #tab PCSTR  #tab str #tab #value;}
                } else	{let value = writer.value(        &writer.reader.constant_value(constant));
                      	quote! {#name #tab PCWSTR #tab str #tab #value;}}
            } else    	{let value_t = writer.typed_value(&writer.reader.constant_value(constant));
                      	quote! {#name #tab #value_t;}}
                      	// pub const D3DKMT_SUBKEY_DX9: PCWSTR = "DX9";
                      	// p!("¦{:?}¦",writer.reader.constant_value(constant)); //¦String("DX9")¦
                      	// p!("¦{}¦",value); //¦"DX9"¦ patched→ ¦DX9¦
                      	// p!("{:?}",value); TokenStream("i32 = 8i32")
                      	// public const uint DXGK_MIN_PAGE_TABLE_LEVEL_COUNT = 2u;
                      	// public const int D3DCLEAR_COMPUTERECTS = 8;
                      	// let value = TokenStream(parse_lit(&value.to_string())); // 1i32 → 1 (done at typed_value)
        } else {
            let kind = writer.type_default_name(&ty);
            let value = writer.value(&writer.reader.constant_value(constant));
            let underlying_type = type_underlying_type(writer.reader, &ty);

            // pub const MaxKeySetInfoClass: KEY_SET_INFORMATION_CLASS
            // println!("constant={:?} value_pre={:?} constant_type={:?} underlying_type={:?}",&constant,&value,&constant_type,&underlying_type);
            //constant=Constant(Row{row:5253,file:2}) value=TokenStream("2i32") constant_type=I32 underlying_type=I32

            let value = if underlying_type == constant_type {
                                             // value
                                            TokenStream(parse_lit(&value.to_string()))
            } else if writer.std && underlying_type == Type::ISize {
                  	 quote! { ::core::ptr::invalid_mut(#value as _) } // todo: convert to actual value
                  	 // quote! { ::core::ptr::invalid_mut(#value as _) } // ::core::ptr::invalid_mut: expresses that the pointer is not associated with any actual allocation and is little more than a usize address in disguise.
            } else	{quote! {                          #value as _ }};

            // println!("value_pos={:?}",&value); // 2

            if !writer.sys && type_has_replacement(writer.reader, &ty) { //HRESULT|PCSTR|PCWSTR|has_attr(NativeTypedefAttribute)|TypeKind::Enum
                  	//quote! {#name #tab #kind #tab #kind(#value);}
                  	let type_prim = type_to_primitive(writer.reader, &ty);
                  	 quote! {#name #tab #kind #tab #type_prim #tab #value;}
            } else	{quote! {#name #tab #kind #tab _          #tab #value;}}
        }
    } else if let Some(guid) = field_guid(writer.reader, def) {
        let value = writer.guid(&guid);
        // pub const GUID_DEVINTERFACE_GRAPHICSPOWER: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xea5c6870_e93c_4588_bef1_fec42fc9429a);
        // GUID_DEVINTERFACE_GRAPHICSPOWER GUID {ea5c6870-e93c-4588-bef1-fec42fc9429a}
        let guid = writer.type_name(&Type::GUID);
        quote! {#name #tab #guid #tab str #tab #value;}
    } else if let Some((value, nm_val)) = initializer(writer, def) {
        let kind = writer.type_default_name(&ty);
        let mut result = quote! {};
        let val = quote! {#name #tab #kind #tab _ #tab {#value};}; result.combine(&val); // combo type, no primitive
        if nm_val.len() > 0 {
            for (k,(v,type_nm,type_prim)) in nm_val {
                // println!("v={:?} type_nm={:?} type_prim={:?}",&v,&type_nm,&type_prim); // v="{c50a3f10-aa5c-4247-b830-d6a6f8eaa310}" type_nm="GUID" type_prim="str"
                let val = quote! {#name _ #k #tab #type_nm #tab #type_prim #tab #v;}; result.combine(&val);
                // DEVPKEY_Device_ActivityId_fmtid¦GUID¦str¦{c50a3f10-aa5c-4247-b830-d6a6f8eaa310}
            }
        }
        result
    } else {quote! {}}
}

fn initializer(writer: &Writer, def: Field) -> Option<(TokenStream,HashMap<String,(String,String,String)>)> {
    let Some(value) = constant(writer, def) else { return None; };
    let mut input = value.as_str();
    let Type::TypeDef(def, _) = writer.reader.field_type(def, None) else { unimplemented!();};
    let mut result = quote! {};
    let mut result_map:HashMap<String,(String,String,String)>	= HashMap::new();

    for field in writer.reader.type_def_fields(def) {
        let (value, rest, nm_val_type) = field_initializer(writer, field, input);
        // println!("field {:?}, input {:?}, rest {:?}, value {:?}", &field, &input, &rest, &value);
        // field Field(Row{row:119991,file:1}), input "{3305783056, 43612, 16967, 184, 48, 214, 166, 248, 234, 163, 16}, 4", rest ", 4", value TokenStream("fmtid:{c50a3f10-aa5c-4247-b830-d6a6f8eaa310},")
        // field Field(Row{row:119992,file:1}), input ", 4", rest "", value TokenStream("pid:4,")
        input = rest;
        result.combine(&value);
        if let Some((nm,val,type_nm,type_prim)) = nm_val_type {
            result_map.insert(nm.as_str().to_string(),
                (val.as_str().to_string(),type_nm.to_string(),type_prim.to_string(),));
        } else {}
    }

    Some((result, result_map))
}

fn field_initializer<'a>(writer: &Writer, field: Field, input: &'a str) -> (TokenStream, &'a str, Option<(TokenStream,TokenStream,String,String)>) {
    let name = to_ident(writer.reader.field_name(field));

    match writer.reader.field_type(field, None) {
        Type::GUID => {
            let (literals, rest) = read_literal_array(input, 11);
            // println!("literals {:?}, rest {:?}", literals, rest);
              // [Constant("{3305783056,43612,16967,184,48,214,166,248,234,163,16},4")]public static DEVPROPKEY DEVPKEY_Device_ActivityId;
              //   literals["3305783056","43612","16967","184","48","214","166","248","234","163","16"],rest",4"
            let value = writer.guid(&GUID::from_string_args(&literals));
            let type_ = Type::GUID;
            let type_nm = "GUID".to_string();
            let type_prim = type_to_primitive(writer.reader, &type_);
            // println!("name={:?} value= {:?} type_nm={:?} type_prim={:?}",&name,&value,&type_nm,&type_prim); //name=TokenStream("fmtid") value= TokenStream("{c50a3f10-aa5c-4247-b830-d6a6f8eaa310}") type_nm="GUID" type_prim="str"
            (quote! { #name : #value, }, rest, Some((name,value,type_nm,type_prim.to_string())))
        }
        Type::Win32Array(_, len) => {
            let (literals, rest) = read_literal_array(input, len);
            let literals = literals.iter().map(|literal| TokenStream::from(*literal));
            (quote! { #name: [#(#literals,)*], }, rest, None)
        }
        _ => {
            let (literal, rest) = read_literal(input);
            let type_ = &writer.reader.field_type(field,None);
            let type_nm = type_prim_to_str(&type_);
            let type_prim = type_to_primitive(writer.reader, &type_);
            // println!("input={:?} literal= {:?} rest={:?} type_={:?} type_nm={:?} type_prim={:?}",&input,&literal,&rest,&type_,&type_nm,&type_prim); //input=", 4" literal= "4" rest="" type_=U32 type_nm="u32" type_prim="u32"
            let literal: TokenStream = literal.into();
            (quote! { #name: #literal, }, rest, Some((name,literal,type_nm.to_string(),type_prim.to_string())))
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
