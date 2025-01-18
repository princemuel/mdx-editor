use uint::construct_uint;

// Construct an unsigned 256-bit integer
// consisting of 4 x 64-bit words
construct_uint! { pub struct U256(4); }

pub mod crypto;
pub mod interfaces;
pub mod sha256;
pub mod util;
