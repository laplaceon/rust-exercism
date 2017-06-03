pub fn raindrops(n: i32) -> String {
	let mut sound = "".to_string();
	let mut b = false;
	if n % 3 == 0 {
		sound.push_str("Pling");
		b = true;
	}
	if n % 5 == 0 {
		sound.push_str("Plang");
		b = true;
	}
	if n % 7 == 0 {
		sound.push_str("Plong");
		b = true;
	}
	
	if !b {
		return n.to_string();
	}
	
	return sound;
}