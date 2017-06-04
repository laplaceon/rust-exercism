pub fn is_pangram(sentence: &str) -> bool {
	let mut alphabet_map: [bool; 26] = [false; 26];
	{
		let note_letter = |c| {
			let i = c as i32;
			if i >= 97 && i <= 122 {
				// Translate the index so that it starts from 0
				alphabet_map[(i-97) as usize] = true;
			}
		};
		
		// Have to use a count to consume the iterator
		sentence.to_lowercase().chars().map(note_letter).count();
	}
	
	for i in 0..26 {
		if !alphabet_map[i] {
			return false;
		}
	}
	
	true
}