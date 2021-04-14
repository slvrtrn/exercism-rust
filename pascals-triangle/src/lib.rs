pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>
}

fn fac(n: u32) -> u32 {
    return if n < 2 {
        1
    } else {
        n * fac(n - 1)
    };
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut rows: Vec<Vec<u32>> = Vec::with_capacity(row_count as usize);
        for n in 0..row_count {
            let mut cols: Vec<u32> = Vec::with_capacity((n + 1) as usize);
            for k in 0..=n {
                cols.push(fac(n) / (fac(k) * fac(n - k)))
            }
            rows.push(cols);
        }
        PascalsTriangle { rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
