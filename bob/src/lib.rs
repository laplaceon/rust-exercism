pub fn reply(s: &str) -> &'static str {
	let s = String::from(s);
	let s_as_chars = s.chars();
	
	if s_as_chars.count() == 0 {
		return "Fine. Be that way!"
	}
	
	let mut s_as_chars = s.chars();
	
	let mut last_char = 'a';
	let mut all_caps = true;
	while let Some(c) = s_as_chars.next() {
		last_char = c;
		if (c as i32 >= 97) && (c as i32 <= 122) {
			all_caps = false;
		}
	}
	
	if last_char == '?' {
		return "Sure.";
	}
	
	if all_caps {
		return "Whoa, chill out!";
	}
	
	return "Whatever."
}