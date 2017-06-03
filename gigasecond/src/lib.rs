extern crate chrono;
use chrono::*;
use std::ops::Add;

pub fn after(dt: DateTime<UTC>) -> DateTime<UTC> {
	let gs = Duration::seconds(1000000000);
	
	return dt.add(gs);
}