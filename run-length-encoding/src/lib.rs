// Not my best work, I have seen a better solution using iterator.fold() and I understand how that's used now and will review passing closures
// as arguments

pub fn decode(s: &str) -> String {
	let mut decoded = String::new();
	
	let s = s.to_string();
	let mut current_n = "0".to_string();
	
	let mut chars = s.chars();
	
	while let Some(c) = chars.next() {
		if c.is_numeric() {
			current_n.push(c);
		} else {
			let n: usize = current_n.parse().unwrap();
			if n > 1 {
				let mut c_str = String::new();
				c_str.push(c);
				decoded.push_str(c_str.repeat(n).as_str());
			} else {
				decoded.push(c);
			}
			current_n = "0".to_string();
		}
	}
	
	decoded
}

pub fn encode(s: &str) -> String {
	let mut encoded = String::new();
	let s = s.to_string();
	let mut chars = s.chars();
	if let Some(mut last_char) = chars.next() {
		let mut current_run_length = 1;
		while let Some(c) = chars.next() {
			if c == last_char {
				current_run_length += 1;
			} else if current_run_length > 1 {
				encoded.push_str(format!("{}{}", current_run_length, last_char).as_str());
				current_run_length = 1;
			} else {
				encoded.push_str(format!("{}", last_char).as_str());
				current_run_length = 1;
			}
			
			last_char = c;
		}
		
		if current_run_length > 1 {
			encoded.push_str(format!("{}{}", current_run_length, last_char).as_str());
		} else {
			encoded.push_str(format!("{}", last_char).as_str());
		}
		
	}
	
	encoded
}