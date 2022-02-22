use needleman_wunsch::NeedlemanWunsch;

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct Hirschberg {
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
impl Hirschberg {
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

    pub fn hirschberg(&self) -> (String, String) {
        let mut alignment_a = String::new();
        let mut alignment_b = String::new();

        if self.first.len() == 0 {
            for i in 0..self.second.len() {
                alignment_a.push_str(&String::from("-"));
                alignment_b.push_str(&String::from(self.second.chars().nth(i).unwrap()));
            }
            alignment_a = alignment_a.chars().rev().collect();
            alignment_b = alignment_b.chars().rev().collect();
        } else if self.second.len() == 0 {
            for i in 0..self.first.len() {
                alignment_a.push_str(&String::from(self.first.chars().nth(i).unwrap()));
                alignment_b.push_str(&String::from("-"));
            }
            alignment_a = alignment_a.chars().rev().collect();
            alignment_b = alignment_b.chars().rev().collect();
        } else if self.first.len() == 1 || self.second.len() == 1 {
            let needleman = NeedlemanWunsch::new(
                self.first.clone(),
                self.second.clone(),
                self.gap_score,
                self.blossum.clone(),
                self.mapping,
            );

            let (alignment_a_nw, alignment_b_nw, _) = needleman.compute();

            alignment_a = alignment_a_nw;
            alignment_b = alignment_b_nw;
        } else {
            let xmid = self.first.len() / 2;

            let score_l = Self::compute(
                self.first.clone()[..xmid].to_string(),
                self.second.clone(),
                self.gap_score,
                self.blossum.clone(),
                self.mapping,
            );

            let score_r = Self::compute(
                self.first.clone()[xmid..].chars().rev().collect(),
                self.second.clone().chars().rev().collect(),
                self.gap_score,
                self.blossum.clone(),
                self.mapping,
            );

            let ymid: Vec<i32> = score_l
                .iter()
                .zip(score_r.iter().rev())
                .map(|(&l, &r)| l + r)
                .collect();

            let &ymax = ymid.iter().max().unwrap();
            let ymid = ymid.iter().position(|&a| a == ymax).unwrap();

            let hirschberg_l = Self {
                first: self.first.clone()[..xmid].to_string(),
                second: self.second.clone()[..ymid].to_string(),
                gap_score: self.gap_score,
                blossum: self.blossum.clone(),
                mapping: self.mapping,
            };

            let hirschberg_r = Self {
                first: self.first.clone()[xmid..].to_string(),
                second: self.second.clone()[ymid..].to_string(),
                gap_score: self.gap_score,
                blossum: self.blossum.clone(),
                mapping: self.mapping,
            };

            let (al, bl) = hirschberg_l.hirschberg();
            let (ar, br) = hirschberg_r.hirschberg();

            alignment_a.push_str(&al);
            alignment_a.push_str(&ar);

            alignment_b.push_str(&bl);
            alignment_b.push_str(&br);
        }

        (alignment_a, alignment_b)
    }

    fn compute(
        first: String,
        second: String,
        gap_score: i32,
        blossum: Vec<Vec<i32>>,
        mapping: &'static str,
    ) -> Vec<i32> {
        let mut score = vec![vec![0; second.len() + 1]; 2];

        for i in 1..=second.len() {
            score[0][i] = score[0][i - 1] + gap_score;
        }

        for i in 1..=first.len() {
            score[1][0] = score[0][0] + gap_score;

            for j in 1..=second.len() {
                let blossum_i = mapping
                    .find(first.chars().nth(i.checked_sub(1).unwrap()).unwrap())
                    .unwrap();

                let blossum_j = mapping
                    .find(second.chars().nth(j.checked_sub(1).unwrap()).unwrap())
                    .unwrap();

                let sub = blossum[blossum_i][blossum_j];

                let score_sub = score[0][j - 1] + sub;
                let score_del = score[0][j] + gap_score;
                let score_ins = score[1][j - 1] + gap_score;

                score[1][j] = max!(score_sub, score_del, score_ins);
            }

            score[0] = score[1].clone();
        }

        score[1].clone()
    }
}
