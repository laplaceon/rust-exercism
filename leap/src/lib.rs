pub fn is_leap_year(y: i32) -> bool {
	if (y % 4 == 0) && ((y % 100 != 0) || (y % 400 == 0)) {
		return true;
	}
	
	return false;
}