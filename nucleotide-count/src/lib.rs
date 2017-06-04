use std::collections::HashMap;

pub fn count(base: char, strand: &str) -> Result<usize, &str> {
	let valid_base = |b| b == 'A' || b == 'T' || b == 'C' || b == 'G';
	
	if !valid_base(base) {
		return Err("This isn't an actual base.");
	}
	
	let mut n = 0;
	
	let mut char_iter = strand.chars();
	while let Some(x) = char_iter.next() {
		if !valid_base(x) {
			return Err("This base in the strand isn't real.");
		}
		
		if x == base {
			n += 1;
		}
	}
		
	Ok(n as usize)
}

pub fn nucleotide_counts(strand: &str) -> Result<HashMap<char, usize>, &str> {
	let bases = vec!['A', 'T', 'C', 'G'];
	
	let mut map: HashMap<char, usize> = HashMap::new();
	
	for i in 0..4 {
		let base = bases[i];
		if let Ok(n) = count(base, strand) {
			map.insert(base, n);
		}
		else {
			return Err("There was an error!");
		}
	}
	
	Ok(map)
}