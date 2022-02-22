#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct NeedlemanWunsch {
    first: String,
    second: String,
    gap_score: i32,

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
impl NeedlemanWunsch {
    pub fn new(
        first: String,
        second: String,
        gap_score: i32,
        blossum: Vec<Vec<i32>>,
        mapping: &'static str,
    ) -> Self {
        Self {
            first,
            second,
            gap_score,
            blossum,
            mapping,
        }
    }

    pub fn compute(&self) -> (String, String, i32) {
        let mut matrix = vec![vec![0; self.second.len() + 1]; self.first.len() + 1];

        for i in 0..self.first.len() + 1 {
            matrix[i][0] = self.gap_score * (i as i32);
        }

        for i in 0..self.second.len() + 1 {
            matrix[0][i] = self.gap_score * (i as i32);
        }

        for i in 1..self.first.len() + 1 {
            for j in 1..self.second.len() + 1 {
                let blossum_i = self
                    .mapping
                    .find(self.first.chars().nth(i.checked_sub(1).unwrap()).unwrap())
                    .unwrap();

                let blossum_j = self
                    .mapping
                    .find(self.second.chars().nth(j.checked_sub(1).unwrap()).unwrap())
                    .unwrap();

                let equal = matrix[i - 1][j - 1] + self.blossum[blossum_i][blossum_j];

                let delete = matrix[i - 1][j] + self.gap_score;
                let insert = matrix[i][j - 1] + self.gap_score;

                matrix[i][j] = max!(equal, delete, insert);
            }
        }

        let mut alignment_a = String::new();
        let mut alignment_b = String::new();

        let mut i = self.first.len();
        let mut j = self.second.len();

        while i > 0 && j > 0 {
            let blossum_i = self
                .mapping
                .find(self.first.chars().nth(i.checked_sub(1).unwrap()).unwrap())
                .unwrap();

            let blossum_j = self
                .mapping
                .find(self.second.chars().nth(j.checked_sub(1).unwrap()).unwrap())
                .unwrap();

            let equal = matrix[i - 1][j - 1] + self.blossum[blossum_i][blossum_j];

            let ai = self.mapping.chars().nth(blossum_i).unwrap().to_string();
            let bi = self.mapping.chars().nth(blossum_j).unwrap().to_string();

            if i > 0 && j > 0 && matrix[i][j] == equal {
                alignment_a.push_str(&ai);
                alignment_b.push_str(&bi);

                i -= 1;
                j -= 1;
            } else if i > 0 && matrix[i][j] == matrix[i - 1][j] + self.gap_score {
                alignment_a.push_str(&ai);
                alignment_b.push_str(&String::from("-"));

                i -= 1;
            } else {
                alignment_a.push_str(&String::from("-"));
                alignment_b.push_str(&bi);

                j -= 1;
            }
        }

        let alignment_a = alignment_a.chars().rev().collect();
        let alignment_b = alignment_b.chars().rev().collect();
        let score = matrix[self.first.len()][self.second.len()];

        (alignment_a, alignment_b, score)
    }
}
