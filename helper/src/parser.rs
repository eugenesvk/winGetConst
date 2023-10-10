use crate::alias::*;
// pub allows use in other files
pub fn rem_first(value: &str) -> &str {
  let mut chars = value.chars();
  chars.next();
  // chars.next_back();
  chars.as_str()
}
pub fn parse_lit(num:&str) -> String {
  use litrs::{Literal};
  let mut sign   	= "".to_string();
  let     nonum  	= "✗".to_string();
  let mut num_pos	= num.to_string();
  if num.starts_with("-") {
    sign   	+= "-";
    num_pos	 = rem_first(&num).to_string();
  }
  let lit_try	= Literal::parse(num_pos); //.expect(&format!("✗parse literal ‘{}’",&num));
  match lit_try {
    Ok(lit) => { match lit {
      // 27f32 bugs as an integer https://github.com/LukasKalbertodt/litrs/issues/14
      Literal::Integer	(lit) => { //https://doc.rust-lang.org/stable/std/primitive/index.html
        // avoids matching by suffix, use max Unsigned value to avoid overflowing of -MAXu128
        match lit.value::<u128>().map(|n| n.to_string()) {Some(n) => sign + &n.to_string(), None => nonum}

        // match lit.value::<u128>()	{Some(n) => sign + &n.to_string(), None => nonum},
        // p!("Integer raw_input {:?}",lit.raw_input());
        // todo: change to proper types one weird Rust bug can be avoided witn a .lenient version
        // https://docs.rs/litrs/latest/litrs/struct.IntegerLit.html
        /*
        match lit.suffix() {
           // ↓ would become floats?
           "f32"	=> match lit.value::< u32>()	{Some(n) => sign + &n.to_string(), None => nonum},
           "f64"	=> match lit.value::< u64>()	{Some(n) => sign + &n.to_string(), None => nonum},
           // ↓ −128i8 would overflow without the sing: 128i8 doesn't fit the max 127i8
         //  "i8"	=> match lit.value::<  i8>() 	{Some(n) => sign + &n.to_string(), None => nonum},
         // "i16"	=> match lit.value::< i16>() 	{Some(n) => sign + &n.to_string(), None => nonum},
         // "i32"	=> match lit.value::< i32>() 	{Some(n) => sign + &n.to_string(), None => nonum},
         // "i64"	=> match lit.value::< i64>() 	{Some(n) => sign + &n.to_string(), None => nonum},
            "i8" 	=> match lit.value::<  u8>() 	{Some(n) => sign + &n.to_string(), None => nonum},
           "i16" 	=> match lit.value::< u16>() 	{Some(n) => sign + &n.to_string(), None => nonum},
           "i32" 	=> match lit.value::< u32>() 	{Some(n) => sign + &n.to_string(), None => nonum},
           "i64" 	=> match lit.value::< u64>() 	{Some(n) => sign + &n.to_string(), None => nonum},
          "i128" 	=> match lit.value::<u128>() 	{Some(n) => sign + &n.to_string(), None => nonum},
          "isize"	=> match lit.value::<usize>()	{Some(n) => sign + &n.to_string(), None => nonum},
            "u8" 	=> match lit.value::<  u8>() 	{Some(n) => sign + &n.to_string(), None => nonum},
           "u16" 	=> match lit.value::< u16>() 	{Some(n) => sign + &n.to_string(), None => nonum},
           "u32" 	=> match lit.value::< u32>() 	{Some(n) => sign + &n.to_string(), None => nonum},
           "u64" 	=> match lit.value::< u64>() 	{Some(n) => sign + &n.to_string(), None => nonum},
          "u128" 	=> match lit.value::<u128>() 	{Some(n) => sign + &n.to_string(), None => nonum},
          "usize"	=> match lit.value::<usize>()	{Some(n) => sign + &n.to_string(), None => nonum},
          ""     	=> match lit.value::< u64>() 	{Some(n) => sign + &n.to_string(), None => nonum},
          // _   	=> match lit.value::<u128>() 	{Some(n) => sign + &n.to_string(), None => nonum},
          _      	=> nonum,
        }*/
      }
      Literal::Float	(lit) => {
        // p!("Float part {:?}",lit.number_part());
        match lit.suffix() {
          "f32"	=> match lit.number_part().parse::<f32>()	{Ok(n) => sign + &n.to_string(), Err(_) => nonum},
          "f64"	=> match lit.number_part().parse::<f64>()	{Ok(n) => sign + &n.to_string(), Err(_) => nonum},
          ""   	=> match lit.number_part().parse::<f64>()	{Ok(n) => sign + &n.to_string(), Err(_) => nonum},
          _    	=> nonum,
        }}
      Literal::Bool      	(lit) => { lit.to_string() }
      Literal::Char      	(lit) => { lit.to_string() }
      Literal::String    	(lit) => { lit.to_string() }
      Literal::Byte      	(lit) => { lit.to_string() }
      Literal::ByteString	(lit) => { lit.to_string() }
    }},
    Err(ParseError) => "_".to_string(),
  }
}

// {81d0bfd5-6afe-48c2-99c0-95a08f97c5da}	DXVA_COPPQueryConnectorType
// let myguid = ahkGUID::from_u128(0x81d0bfd5_6afe_48c2_99c0_95a08f97c5da);
                            // 81d0bfd5-6afe-48c2-99c0-95a08f97c5da
// let myguids = format!("{:?}",myguid);
// p!("{}",myguids);

// #[repr(C)] pub struct ahkGUID {
//   pub n1:u32,
//   pub n2:u16,
//   pub n3:u16,
//   pub n4:u16,
//   pub n5:u16,
//   pub n6:u32,}
// impl ::core::marker::Copy for ahkGUID {}
// impl ::core::clone::Clone for ahkGUID {fn clone(&self) -> Self { *self }}
// impl ahkGUID {
//    pub const fn from_u128(uuid:u128) -> Self {
//     Self {
//       n1: (uuid >> 96             ) as u32,
//       n2: (uuid >> 80 & 0x____ffff) as u16,
//       n3: (uuid >> 64 & 0x____ffff) as u16,
//       n4: (uuid >> 48 & 0x____ffff) as u16,
//       n5: (uuid >> 32 & 0x____ffff) as u16,
//       n6: (uuid       & 0xffffffff) as u32,
//       }}
//  }
// impl std::fmt::Debug for ahkGUID { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//   write!(f, "{{{:08x?}-{:04x?}-{:04x?}-{:04x?}-{:04x?}{:08x?}}}",
//     self.n1,self.n2,self.n3,self.n4,self.n5,self.n6)
// }}
