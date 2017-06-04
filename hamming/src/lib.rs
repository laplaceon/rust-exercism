pub fn hamming_distance(s1: &str, s2: &str) -> Result<i32, &'static str> {
	let s1 = String::from(s1);
	let s2 = String::from(s2);
	
	if s1.len() != s2.len() {
		return Err("Not point mutation");
	}
	
	let comparator = s1.chars().zip(s2.chars());
	
	let errors = comparator.fold(0, |acc, (c1, c2)| if c1 != c2 { acc + 1 } else { acc });
	
	Ok(errors)
}