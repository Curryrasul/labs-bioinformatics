mod utils;
#[allow(unused_imports)]
use utils::get_blossum;

mod nw_af;
#[allow(unused_imports)]
use nw_af::NeedlemanWunschAffine;

#[allow(dead_code)]
static MAPPING: &str = "ARNDCQEGHILKMFPSTWYVBZX*";

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn nw_affine_test() {
        let first = String::from("GATTACA");
        let second = String::from("GCATGCG");
        let gap_enter = -1;
        let gap_extend = -1;
        let blossum = get_blossum("../blossum.txt");

        let needleman = NeedlemanWunschAffine::new(first, second, gap_enter, gap_extend, blossum, MAPPING);

        let (alignment_a, alignment_b, score) = needleman.compute();

        println!("{}", alignment_a);
        println!("{}", alignment_b);
        println!("{}", score);
    }
}
