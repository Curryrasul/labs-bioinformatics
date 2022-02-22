mod utils;
#[allow(unused_imports)]
use utils::get_blossum;

pub mod nw;
#[allow(unused_imports)]
pub use nw::NeedlemanWunsch;

#[allow(dead_code)]
static MAPPING: &str = "ARNDCQEGHILKMFPSTWYVBZX*";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nw_test_blossum() {
        let first = String::from("GATTACA");
        let second = String::from("GCATGCG");
        let gap_score = -1;
        let blossum = get_blossum("../blossum.txt");

        let needleman = NeedlemanWunsch::new(first, second, gap_score, blossum, MAPPING);

        let (alignment_a, alignment_b, score) = needleman.compute();

        println!("{}", alignment_a);
        println!("{}", alignment_b);
        println!("{}", score);
    }

    #[test]
    fn nw_test_default() {
        let first = String::from("GATTACA");
        let second = String::from("GCATGCG");
        let gap_score = -1;

        let mut m = vec![vec![-1; 24]; 24];
        for i in 0..24 {
            m[i][i] = 1;
        }

        let needleman = NeedlemanWunsch::new(first, second, gap_score, m, MAPPING);

        let (alignment_a, alignment_b, score) = needleman.compute();

        println!("{}", alignment_a);
        println!("{}", alignment_b);
        println!("{}", score);
    }
}
