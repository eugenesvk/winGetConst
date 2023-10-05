struct Years(i32);
const fn five() -> i32 { 5 }
pub const MIN                                 	:          usize	= 11     ;
pub const PUB_CONST_IN_GLOBAL_SIZE            	:          usize	= 16     ;
pub const PUB_CONST_IN_GLOBAL_LOG_AS          	: &'static str  	= "batch";
pub const PUB_CONST_IN_GLOBAL_MIN_SIZE        	:          usize	= MIN    ;
pub const PUB_CONST_IN_GLOBAL_EXPR2_2         	:          i32  	= 2+2    ;
pub const PUB_CONST_IN_GLOBAL_FN_FIVE         	:          i32  	= five() ;
pub const PUB_CONST_IN_GLOBAL                 	:          i32  	= 54321  ;
pub const PUB_CONST_IN_GLOBAL_WILL_REMOVE     	:          i32  	= 981    ;
pub const PUB_CONST_IN_GLOBAL_WILL_CHANGE_VAL 	:          i32  	= 982    ;
pub const PUB_CONST_IN_GLOBAL_WILL_CHANGE_TYPE	:          Years	= Years(5312);
// pub const PUB_CONST_IN_GLOBAL_WILL_RENAME: i32 = 0;
// pub const PUB_CONST_IN_GLOBAL_WILL_BE_STATIC: i32 = 0;
// pub const PUB_CONST_IN_GLOBAL_WILL_BE_PRIVATE_CONST: i32 = 0;
// pub const PUB_CONST_IN_GLOBAL_WILL_BE_PRIVATE_STATIC: i32 = 0;

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
