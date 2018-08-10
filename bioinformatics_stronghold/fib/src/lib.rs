pub fn rabbits(n: u64, k: u64) -> u64 {
    match n {
        1 => 1,
        2 => 1,
        n => (rabbits(n - 2, k) * k) + rabbits(n - 1, k),
    }
}

#[cfg(test)]
mod rabbits {
    use super::*;

    #[test]
    fn month_1() {
        let n = 1;
        let k = 3;
        assert_eq!(rabbits(n, k), 1);
    }

    #[test]
    fn month_2() {
        let n = 2;
        let k = 3;
        assert_eq!(rabbits(n, k), 1);
    }

    #[test]
    fn month_3() {
        let n = 3;
        let k = 3;
        assert_eq!(rabbits(n, k), 4);
    }

    #[test]
    fn month_4() {
        let n = 4;
        let k = 3;
        assert_eq!(rabbits(n, k), 7);
    }

    #[test]
    fn month_5() {
        let n = 5;
        let k = 3;
        assert_eq!(rabbits(n, k), 19);
    }
}
