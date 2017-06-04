pub fn is_valid(code: &str) -> bool {
	let cleaned: String = code.chars().filter(|&x| x != ' ').collect();
	
	if cleaned.len() <= 1 || cleaned.chars().any(|c| !c.is_numeric()) {
		return false;
	}
	
	let luhn_modifier = |(i, x): (usize, char)| -> u32 {
		if let Some(x) = x.to_digit(10) {
			if i % 2 == 0 {
				return x;
			} else {
				let x = x * 2;
				if x > 9 {
					return x - 9;
				} else {
					return x;
				}
			}
		}
		return 0;
	};
	
	if cleaned.chars().rev().enumerate().map(luhn_modifier).sum::<u32>() % 10 == 0 {
		true
	} else {
		false
	}
}