use crate::U256;

// initial reward in bitcoin - multiply by 10^8 to get satoshis
pub const INITIAL_REWARD: u64 = 50;
// halving interval in blocks
pub const HALVING_INTERVAL: u64 = 210;
// ideal block time in seconds || BTC is 600
pub const IDEAL_BLOCK_TIME: u64 = 10;
// minimum target in little endian
pub const MIN_TARGET: U256 = U256([
    0xFFFF_FFFF_FFFF_FFFF,
    0xFFFF_FFFF_FFFF_FFFF,
    0xFFFF_FFFF_FFFF_FFFF,
    0x0000_FFFF_FFFF_FFFF,
]);
// difficulty update interval in blocks || BTC is 2016
pub const DIFFICULTY_UPDATE_INTERVAL: u64 = 50; //
