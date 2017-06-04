pub fn lsp(series: &str, n: usize) -> Result<u32, &str> {
	if series.len() < n {
		return Err("The series is not long enough.");
	}
	
	if n == 0 {
		return Ok(1);
	}
	
	let mut num_vec: Vec<u32> = Vec::new();
	
	let mut char_iter = series.chars();
	
	while let Some(c) = char_iter.next() {
		if let Some(i) = c.to_digit(10) {
			num_vec.push(i);
		} else {
			return Err("There was a non digit.");
		}
	}
	
	let num_slice = &num_vec[..];
	
	if let Some(i) = num_slice.windows(n).map(|x| x.iter().product::<u32>()).max() {
		Ok(i)
	} else {
		Err("Could not calculate max.")
	}
}