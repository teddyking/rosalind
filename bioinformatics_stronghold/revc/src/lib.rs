pub fn reverse_complement(input: &str) -> String {
    input.chars().map(|c| complement(c)).rev().collect()
}

fn complement(c: char) -> char {
    match c {
        'A' => 'T',
        'C' => 'G',
        'G' => 'C',
        'T' => 'A',
        _ => '_',
    }
}

#[cfg(test)]
mod reverse_complement {
    use super::*;

    #[test]
    fn happy_path() {
        let input = "AAAACCCGGT";
        assert_eq!(reverse_complement(input), "ACCGGGTTTT");
    }

    #[test]
    fn empty_path() {
        let input = "";
        assert_eq!(reverse_complement(input), "");
    }

    #[test]
    fn another_path() {
        let input = "AAAClol";
        assert_eq!(reverse_complement(input), "___GTTT");
    }
}
