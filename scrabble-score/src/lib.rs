pub fn score(s: &str) -> i32 {
	let score_character = |x| match x {
		'a' | 'e' |	'i' | 'o' | 'u' | 'l' | 'n' | 'r' | 's' | 't' => 1,
		'd' | 'g' => 2,
		'b' | 'c' | 'm' | 'p' => 3,
		'f' | 'h' | 'v' | 'w' | 'y' => 4,
		'k' => 5,
		'j' | 'x' => 8,
		'q' | 'z' => 10,
		_ => 0,
	};
	
	s.to_lowercase().chars().map(score_character).fold(0, |acc, x| acc + x)
}