pub fn acgt_counts(input: &str) -> (usize, usize, usize, usize) {
    let mut a = 0;
    let mut c = 0;
    let mut g = 0;
    let mut t = 0;

    for ch in input.chars() {
        match ch {
            'A' => a += 1,
            'C' => c += 1,
            'G' => g += 1,
            'T' => t += 1,
            _ => {}
        }
    }

    (a, c, g, t)
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
