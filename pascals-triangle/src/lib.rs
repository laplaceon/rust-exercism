#[derive(Debug)]
pub struct PascalsTriangle {
	pub rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
		let check_edge = |a, b| a == b || b == 1;
		let mut t: Vec<Vec<u32>> = Vec::new();
		for i in 0..row_count {
			let mut row: Vec<u32> = Vec::new();
			for j in 0..i+1 {
				if check_edge(i + 1, j + 1) {
					row.push(1);
				} else {
					let sum = t[(i - 1) as usize][(j - 1) as usize] + t[(i - 1) as usize][j as usize];
					row.push(sum);
				}
			}
			t.push(row);
		}
		
		PascalsTriangle { rows: t }
    }

	// Unfortunately, my implementation involved me changing this from borrowing to owning
	// I checked the example answer and their implementation crossed my mind but I thought it would be a little overkill
	// to perform the triangle creation everything rows() was called, so I didn't do it that way
    pub fn rows(self) -> Vec<Vec<u32>> {
        self.rows
    }
}
