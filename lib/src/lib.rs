use serde::{Deserialize, Serialize};
use uint::construct_uint;

construct_uint! {
   // Construct an unsigned 256-bit integer consisting of 4 x 64-bit words
   #[derive(Serialize, Deserialize)]
   pub struct U256(4);
}

pub mod crypto;
pub mod error;
pub mod interfaces;
pub mod sha256;
pub mod util;
