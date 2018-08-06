pub fn dna_to_rna(input: &str) -> String {
    input.replace("T", "U")
}

#[cfg(test)]
mod dna_to_rna {
    use super::*;

    #[test]
    fn happy_path() {
        let input = "GATGGAACTTGACTACGTAAATT";
        assert_eq!(dna_to_rna(input), "GAUGGAACUUGACUACGUAAAUU");
    }

    #[test]
    fn empty_path() {
        let input = "";
        assert_eq!(dna_to_rna(input), "");
    }

    #[test]
    fn another_path() {
        let input = "somerandomstuff@lolomgT";
        assert_eq!(dna_to_rna(input), "somerandomstuff@lolomgU");
    }
}
