pub fn acgt_counts(input: &str) -> (usize, usize, usize, usize) {
    let a = input.matches("A");
    let c = input.matches("C");
    let g = input.matches("G");
    let t = input.matches("T");

    (a.count(), c.count(), g.count(), t.count())
}

#[cfg(test)]
mod acgt_counts {
    use super::*;

    #[test]
    fn happy_path() {
        let input = "AACCaGGcTTtlolcakesA";
        assert_eq!(acgt_counts(input), (3, 2, 2, 2));
    }

    #[test]
    fn empty_path() {
        let input = "";
        assert_eq!(acgt_counts(input), (0, 0, 0, 0));
    }

    #[test]
    fn another_path() {
        let input = "GC";
        assert_eq!(acgt_counts(input), (0, 1, 1, 0));
    }
}
