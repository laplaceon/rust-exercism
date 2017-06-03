pub fn square_of_sum(i: i32) -> i32 {
	let sum = i * (i + 1) / 2;
	
	sum * sum
}

pub fn sum_of_squares(i: i32) -> i32 {
	let square = |x| x * x;
	
	let mut sum = 0;
	
	for n in 1..i+1 {
		sum += square(n);
	}
	
	sum
}

pub fn difference(i: i32) -> i32 {
	square_of_sum(i) - sum_of_squares(i)
}