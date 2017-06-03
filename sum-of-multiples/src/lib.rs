pub fn sum_of_multiples(i: i32, v: &Vec<i32>) -> i32 {
	let is_factor = |x| v.iter().any(|n| x % n == 0);
	(0..i).fold(0, |acc, x| if is_factor(x) { acc + x } else { acc })
}