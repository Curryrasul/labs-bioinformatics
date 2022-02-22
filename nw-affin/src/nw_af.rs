#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct NeedlemanWunschAffine {
    first: String,
    second: String,
    gap_enter: i32,
    gap_extend: i32,

    blossum: Vec<Vec<i32>>,
    mapping: &'static str,
}

macro_rules! max {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        let y = max!($($z),*);
        if $x > y {
            $x
        } else {
            y
        }
    }}
}

#[allow(dead_code)]
impl NeedlemanWunschAffine {
    pub fn new(
        first: String,
        second: String,
        gap_enter: i32,
        gap_extend: i32,
        blossum: Vec<Vec<i32>>,
        mapping: &'static str,
    ) -> Self {
        Self {
            first,
            second,
            gap_enter,
            gap_extend,
            blossum,
            mapping,
        }
    }

    pub fn compute(&self) -> (String, String, i32) {
        let m = self.first.len();
        let n = self.second.len();

        let inf = 2 * self.gap_enter + (n + m) as i32 * self.gap_extend + 1;

        let mut match_table = vec![vec![0; m + 1]; n + 1];
        let mut insertion_table = vec![vec![0; m + 1]; n + 1];
        let mut deletion_table = vec![vec![0; m + 1]; n + 1];
        let mut result = vec![vec![0; m + 1]; n + 1];

        insertion_table[0][0] = inf;
        deletion_table[0][0] = inf;
        match_table[0][0] = 0;

        for i in 0..n {
            match_table[i + 1][0] = inf;
            insertion_table[i + 1][0] = inf;
            deletion_table[i + 1][0] = self.gap_enter + i as i32 * self.gap_extend;
        }

        for j in 0..n {
            match_table[0][j + 1] = inf;
            insertion_table[0][j + 1] = self.gap_enter + j as i32 * self.gap_extend;
            deletion_table[0][j + 1] = inf;
        }

        for i in 1..=n {
            for j in 1..=m {
                let blossum_i = self
                    .mapping
                    .find(self.first.chars().nth(i.checked_sub(1).unwrap()).unwrap())
                    .unwrap();

                let blossum_j = self
                    .mapping
                    .find(self.second.chars().nth(j.checked_sub(1).unwrap()).unwrap())
                    .unwrap();

                let table_value = self.blossum[blossum_i][blossum_j];

                let tmp_max = max!(
                    match_table[i - 1][j - 1],
                    insertion_table[i - 1][j - 1],
                    deletion_table[i - 1][j - 1]
                );

                match_table[i][j] = tmp_max + table_value;

                insertion_table[i][j] = max!(
                    match_table[i][j - 1] + self.gap_enter,
                    insertion_table[i][j - 1] + self.gap_extend,
                    deletion_table[i][j - 1] + self.gap_enter
                );

                deletion_table[i][j] = max!(
                    match_table[i - 1][j] + self.gap_enter,
                    insertion_table[i - 1][j] + self.gap_enter,
                    deletion_table[i - 1][j] + self.gap_extend
                );

                let maximum = max!(
                    match_table[i][j],
                    insertion_table[i][j],
                    deletion_table[i][j]
                );

                if maximum == match_table[i][j] {
                    result[i][j] = 0;
                } else if maximum == insertion_table[i][j] {
                    result[i][j] = 1;
                } else {
                    result[i][j] = 2;
                }
            }
        }

        let (mut i, mut j) = (n, m);

        let mut align_a = String::new();
        let mut align_b = String::new();

        while i > 0 && j > 0 {
            if result[i][j] == 1 {
                align_a.push_str(&String::from("-"));
                align_b.push_str(&String::from(self.second.chars().nth(j - 1).unwrap()));
                j -= 1;
            } else if result[i][j] == 2 {
                align_a.push_str(&String::from(self.first.chars().nth(i - 1).unwrap()));
                align_b.push_str(&String::from("-"));
                i -= 1;
            } else {
                align_a.push_str(&String::from(self.first.chars().nth(i - 1).unwrap()));
                align_b.push_str(&String::from(self.second.chars().nth(j - 1).unwrap()));
                i -= 1;
                j -= 1;
            }
        }

        if i == 0 && j > 0 {
            align_a.push_str(&String::from_utf8(vec![b'-'; j]).unwrap());
        }

        if j == 0 && i > 0 {
            align_b.push_str(&String::from_utf8(vec![b'-'; i]).unwrap());
        }

        (
            align_a.chars().rev().collect(),
            align_b.chars().rev().collect(), 
            match_table[n - 1][m - 1]
        )
    }
}
