struct Years(i32);
pub const PUB_CONST_IN_GLOBAL: i32 = 54321;
pub const PUB_CONST_IN_GLOBAL_WILL_REMOVE: i32 = 0;
pub const PUB_CONST_IN_GLOBAL_WILL_CHANGE_VAL: i32 = 0;
pub const PUB_CONST_IN_GLOBAL_WILL_CHANGE_TYPE: Years = Years(5312);
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
