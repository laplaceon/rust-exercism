pub fn square(s: u32) -> u64 {
	if s == 0 || s > 64 {
		panic!("Square must be between 1 and 64")
	}
	
    2u64.pow(s - 1)
}

pub fn total() -> u64 {
    // (1..65).fold(0, |acc, x| acc + square(x))
	// Sum is equal to 2^65 - 1 which results in overflow so we subtract 1 from the 2^64 and add it to 2^64
	// Which is the sum as saying 2 * 2^64 (2^65) - 1 but we avoid overflow
	square(64) - 1 + square(64)
}