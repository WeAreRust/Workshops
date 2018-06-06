struct FibSeq (u32, u32);

impl FibSeq {
    fn new() -> Self {
        FibSeq(0, 1)
    }

    #[test]
    fn test_index_of_result() {
        assert_eq!(FibSeq::index_of_result(0), Ok(0));
        assert_eq!(FibSeq::index_of_result(1), Ok(1));
        assert_eq!(FibSeq::index_of_result(13), Ok(7));
        assert_eq!(FibSeq::index_of_result(89), Ok(11));
        assert_eq!(FibSeq::index_of_result(90), Err(90));
        assert_eq!(FibSeq::index_of_result(15000), Err(15000));
    }
    fn index_of_result(expected: u32) -> Result<u32, u32> {
        Self::index_of(expected).ok_or(expected)
    }

    #[test]
    fn test_index_of() {
        assert_eq!(FibSeq::index_of(0), Some(0));
        assert_eq!(FibSeq::index_of(1), Some(1));
        assert_eq!(FibSeq::index_of(13), Some(7));
        assert_eq!(FibSeq::index_of(89), Some(11));
        assert_eq!(FibSeq::index_of(90), None);
        assert_eq!(FibSeq::index_of(15000), None);
    }
    fn index_of(expected: u32) -> Option<u32> {
        let fib = Self::new();

        match expected {
            0 => return Some(0),
            1 => return Some(1),
            _ => (),
        };

        let mut index = 2;
        for num in fib {
            if num == expected {
                return Some(index);
            }

            if num > expected {
                return None;
            }

            index += 1;
        }

        return None;
    }
}

impl std::iter::Iterator for FibSeq {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.0 + self.1;
        self.0 = self.1;
        self.1 = ret;
        Some(ret)
    }
}

fn main() {
    let result = FibSeq::index_of(13);
    println!("{:?}", result);
}
