pub mod ddd {
  pub type WIN32_ERROR = u32;
}
pub type HRESULT = i32;
// pub type HSTRING = *mut ::core::ffi::c_void;
// pub type IUnknown = *mut ::core::ffi::c_void;
// pub type IInspectable = *mut ::core::ffi::c_void;
pub type PSTR  	= *mut u8;
pub type PWSTR 	= *mut u16;
pub type PCSTR 	= *const u8;
pub type PCWSTR	= *const u16;
pub type BSTR  	= *const u16;
pub type SOCKET = usize;

pub type BrowserNavConstants = i32;

pub const INVALID_SOCKET: SOCKET = -1i32 as _;
pub const CRL_DIST_POINT_ERR_CRL_ISSUER_BIT: i32 = -2147483648i32; // overflows on naive litrs parsing
pub const ERROR_DS_DRA_BAD_NC: ddd::WIN32_ERROR = 8440u32; // should remove ddd::
pub const navReserved5: BrowserNavConstants = 536870912i32; // should parse through BrowserNavConstants=i32
// @Windows_sys
#[repr(C)] pub struct GUID {
  pub data1: u32,
  pub data2: u16,
  pub data3: u16,
  pub data4: [u8; 8],
}
impl ::core::marker::Copy for GUID {}
impl ::core::clone::Clone for GUID {fn clone(&self) -> Self { *self }}
impl GUID { pub const fn from_u128(uuid: u128) -> Self { Self {
  data1: (uuid >> 96)         	as u32,
  data2: (uuid >> 80 & 0xffff)	as u16,
  data3: (uuid >> 64 & 0xffff)	as u16,
  data4: (uuid                	as u64).to_be_bytes()
  }}
}
pub const Dot11AdHocManager        	:GUID 	= GUID::from_u128(0xdd06a84f_83bd_4d01_8ab9_2389fea0869e);
pub const IHV_INIT_VS_FUNCTION_NAME	:PCSTR	= s!("Dot11ExtIhvInitVirtualStation");
// @ rustdoc for windows_sys ✗NOT enough info, NO value
// Dot11AdHocManager → "constant":
  // "type":{"resolved_path":{"name":"GUID","id":"0:19:1716","args":{"angle_bracketed":{"args":[],"bindings":[]}}}},
  // "expr":"_",
  // "value":null,
  // "is_literal":false}
// @ rustdoc for windows_sys ✗NOT enough info, NO value
// IHV_INIT_VS_FUNCTION_NAME → "constant":
  // {"type":{"resolved_path":{"name":"PCSTR","id":"0:13:1708","args":{"angle_bracketed":{"args":[],"bindings":[]}}}},
  // "expr":"_",
  // "value":"{alloc155:*constu8}",
  // "is_literal":false}
// @ziggle
// IHV_INIT_VS_FUNCTION_NAME	Dot11ExhvInitVirtualStation


  pub const fn five() -> i32 { 5 }
  pub struct Years(i32);
  trait MyTrait<const MIN: usize> {           // rustdocs default field
    const NUM     : i32          = 16       ; // 16
    const MIN_SIZE: usize        = MIN      ; // "MIN", referring to the other constant's name
    const LOG_AS  : &'static str = "batch"  ; // "\"batch\"", including escaped quotes
    const EXPR2_2 : i32          = 2+2      ; // "_"
    const FN_FIVE : i32          = five()   ; // "_"
    const YEAR    : Years        = Years(42); // "_"
  }

// @ windows_sys
// pub const STR_ADDR_FMT       	:PCWSTR	= w!("(%02x:%02x:%02x:%02x:%02x:%02x)");
// pub const STR_ADDR_FMTA      	:PCSTR 	= s!("(%02x:%02x:%02x:%02x:%02x:%02x)");
// pub const STR_ADDR_FMTW      	:PCWSTR	= w!("(%02x:%02x:%02x:%02x:%02x:%02x)");
// pub const STR_ADDR_SHORT_FMTA	:PCSTR 	= s!("%04x%08x");
// pub const STR_ADDR_SHORT_FMTW	:PCWSTR	= w!("%04x%08x");
// pub const STR_ADDR_SHORT_FMT 	:PCWSTR	= w!("%04x%08x");
  // format code "%04X"
  // X means that it will print an integer, in hexadecimal, large X for large hexadecimal letters
  // 4 means the number will be printed left justified with at least four digits, print spaces if there is less than four digits
  // 0 means that if there is less than four digits it will print leading zeroes.
// @ziggle
// STR_ADDR_FMTA      		(%02x:%02x:%02x:%02x:%02x:%02x)
// STR_ADDR_FMTW      		(%02x:%02x:%02x:%02x:%02x:%02x)
// STR_ADDR_SHORT_FMTA		%04x%08x
// STR_ADDR_SHORT_FMTW		%04x%08x


// struct Years(i32);
// const fn five() -> i32 { 5 }
// pub const MIN                                 	: usize       	= 11     ;
// pub const PUB_CONST_IN_GLOBAL_SIZE            	: usize       	= 16     ;
// pub const PUB_CONST_IN_GLOBAL_LOG_AS          	: &        str	= "batch";
// pub const PUB_CONST_IN_GLOBAL_LOG_ASS         	: &'static str	= "batch";
// pub const PUB_CONST_IN_GLOBAL_MIN_SIZE        	: usize       	= MIN    ;
// pub const PUB_CONST_IN_GLOBAL_EXPR2_2         	: i32         	= 2+2    ;
// pub const PUB_CONST_IN_GLOBAL_FN_FIVE         	: i32         	= five() ;
// pub const PUB_CONST_IN_GLOBAL                 	: i32         	= 54321  ;
// pub const PUB_CONST_IN_GLOBAL_WILL_REMOVE     	: i32         	= 981    ;
// pub const PUB_CONST_IN_GLOBAL_WILL_CHANGE_VAL 	: i32         	= 982    ;
// pub const PUB_CONST_IN_GLOBAL_WILL_CHANGE_TYPE	: Years       	= Years(5312);
// // pub const PUB_CONST_IN_GLOBAL_WILL_RENAME: i32 = 0;
// // pub const PUB_CONST_IN_GLOBAL_WILL_BE_STATIC: i32 = 0;
// // pub const PUB_CONST_IN_GLOBAL_WILL_BE_PRIVATE_CONST: i32 = 0;
// // pub const PUB_CONST_IN_GLOBAL_WILL_BE_PRIVATE_STATIC: i32 = 0;

// trait BatchIterator<const MIN: usize> {
//   const ASSOC_CONST_MIN                       	: usize       	= 11     ;
//   const ASSOC_CONST_IN_GLOBAL_SIZE            	: usize       	= 16     ;
//   const ASSOC_CONST_IN_GLOBAL_LOG_ASS         	: &'static str	= "batch";
//   const ASSOC_CONST_IN_GLOBAL_MIN_SIZE        	: usize       	= MIN    ;
//   const ASSOC_CONST_IN_GLOBAL_EXPR2_2         	: i32         	= 2+2    ;
//   const ASSOC_CONST_IN_GLOBAL_FN_FIVE         	: i32         	= five() ;
//   const ASSOC_CONST_IN_GLOBAL                 	: i32         	= 54321  ;
//   const ASSOC_CONST_IN_GLOBAL_WILL_REMOVE     	: i32         	= 981    ;
//   const ASSOC_CONST_IN_GLOBAL_WILL_CHANGE_VAL 	: i32         	= 982    ;
//   const ASSOC_CONST_IN_GLOBAL_WILL_CHANGE_TYPE	: Years       	= Years(5312);
// }

// pub mod my_module {
//     pub const PUB_CONST_IN_MODULE: i32 = 0;
//     pub const PUB_CONST_IN_MODULE_WILL_REMOVE: i32 = 0;
//     pub const PUB_CONST_IN_MODULE_WILL_RENAME: i32 = 0;
//     pub const PUB_CONST_IN_MODULE_WILL_BE_STATIC: i32 = 0;
//     pub const PUB_CONST_IN_MODULE_WILL_BE_PRIVATE_CONST: i32 = 0;
//     pub const PUB_CONST_IN_MODULE_WILL_BE_PRIVATE_STATIC: i32 = 0;
//     pub const PUB_CONST_IN_MODULE_WILL_RE_EXPORT: i32 = 0;
//     pub const PUB_CONST_IN_MODULE_WILL_RE_EXPORT_STATIC: i32 = 0;

//     pub mod my_module_nested {
//         pub const PUB_CONST_IN_NESTED_MODULE: i32 = 0;
//         pub const PUB_CONST_IN_NESTED_MODULE_WILL_REMOVE: i32 = 0;
//         pub const PUB_CONST_IN_NESTED_MODULE_WILL_RENAME: i32 = 0;
//         pub const PUB_CONST_IN_NESTED_MODULE_WILL_BE_STATIC: i32 = 0;
//         pub const PUB_CONST_IN_NESTED_MODULE_WILL_BE_PRIVATE_CONST: i32 = 0;
//         pub const PUB_CONST_IN_NESTED_MODULE_WILL_BE_PRIVATE_STATIC: i32 = 0;
//         pub const PUB_CONST_IN_NESTED_MODULE_WILL_RE_EXPORT: i32 = 0;
//         pub const PUB_CONST_IN_NESTED_MODULE_WILL_RE_EXPORT_STATIC: i32 = 0;
//     }
// }



pub mod core {
  #[doc(hidden)] pub const fn decode_utf8_char(bytes: &[u8], mut pos: usize) -> Option<(u32, usize)> {
    if bytes.len() == pos {
        return None;
    }
    let ch = bytes[pos] as u32;
    pos += 1;
    if ch <= 0x7f {
        return Some((ch, pos));
    }
    if (ch & 0xe0) == 0xc0 {
        if bytes.len() - pos < 1 {
            return None;
        }
        let ch2 = bytes[pos] as u32;
        pos += 1;
        if (ch2 & 0xc0) != 0x80 {
            return None;
        }
        let result: u32 = ((ch & 0x1f) << 6) | (ch2 & 0x3f);
        if result <= 0x7f {
            return None;
        }
        return Some((result, pos));
    }
    if (ch & 0xf0) == 0xe0 {
        if bytes.len() - pos < 2 {
            return None;
        }
        let ch2 = bytes[pos] as u32;
        pos += 1;
        let ch3 = bytes[pos] as u32;
        pos += 1;
        if (ch2 & 0xc0) != 0x80 || (ch3 & 0xc0) != 0x80 {
            return None;
        }
        let result = ((ch & 0x0f) << 12) | ((ch2 & 0x3f) << 6) | (ch3 & 0x3f);
        if result <= 0x7ff || (0xd800 <= result && result <= 0xdfff) {
            return None;
        }
        return Some((result, pos));
    }
    if (ch & 0xf8) == 0xf0 {
        if bytes.len() - pos < 3 {
            return None;
        }
        let ch2 = bytes[pos] as u32;
        pos += 1;
        let ch3 = bytes[pos] as u32;
        pos += 1;
        let ch4 = bytes[pos] as u32;
        pos += 1;
        if (ch2 & 0xc0) != 0x80 || (ch3 & 0xc0) != 0x80 || (ch4 & 0xc0) != 0x80 {
            return None;
        }
        let result = ((ch & 0x07) << 18) | ((ch2 & 0x3f) << 12) | ((ch3 & 0x3f) << 6) | (ch4 & 0x3f);
        if result <= 0xffff || 0x10ffff < result {
            return None;
        }
        return Some((result, pos));
    }
    None
  }
  #[doc(hidden)] pub const fn utf16_len(bytes: &[u8]) -> usize {
    let mut pos = 0;
    let mut len = 0;
    while let Some((code_point, new_pos)) = decode_utf8_char(bytes, pos) {
        pos = new_pos;
        len += if code_point <= 0xffff { 1 } else { 2 };
    }
    len
  }
}

/// A literal UTF-8 string with a trailing null terminator.
#[macro_export] macro_rules! s {($s:literal) => { ::core::concat!($s,'\0').as_ptr() };}

/// A literal UTF-16 wide string with a trailing null terminator.
#[macro_export] macro_rules! w {
    ($s:literal) => {{
        const INPUT: &[u8] = $s.as_bytes();
        const OUTPUT_LEN: usize = $crate::core::utf16_len(INPUT) + 1;
        const OUTPUT: &[u16; OUTPUT_LEN] = {
            let mut buffer = [0; OUTPUT_LEN];
            let mut input_pos = 0;
            let mut output_pos = 0;
            while let Some((mut code_point, new_pos)) = $crate::core::decode_utf8_char(INPUT, input_pos) {
                input_pos = new_pos;
                if code_point <= 0xffff {
                    buffer[output_pos] = code_point as u16;
                    output_pos += 1;
                } else {
                    code_point -= 0x10000;
                    buffer[output_pos] = 0xd800 + (code_point >> 10) as u16;
                    output_pos += 1;
                    buffer[output_pos] = 0xdc00 + (code_point & 0x3ff) as u16;
                    output_pos += 1;
                }
            }
            &{ buffer }
        };
        OUTPUT.as_ptr()
    }};
}
