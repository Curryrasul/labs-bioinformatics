mod utils;
#[allow(unused_imports)]
use utils::get_blossum;

mod hg;
#[allow(unused_imports)]
use hg::Hirschberg;

#[allow(dead_code)]
static MAPPING: &str = "ARNDCQEGHILKMFPSTWYVBZX*";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hirschberg_test_blossum() {
        let first = String::from("GATTACA");
        let second = String::from("GCATGCG");
        let gap_score = -1;
        let blossum = get_blossum("../blossum.txt");

        let hirsch = Hirschberg::new(first, second, gap_score, blossum, MAPPING);

        let (alignment_a, alignment_b) = hirsch.hirschberg();

        println!("{} {}", alignment_a, alignment_b);
    }
}
