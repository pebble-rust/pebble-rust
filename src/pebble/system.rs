use crate::pebble::internal::functions::interface::*;
use crate::pebble::types::tm;

pub fn get_time() -> usize {
    time()
}

pub fn is_clock_24h() -> bool {
    clock_is_24h_style()
}