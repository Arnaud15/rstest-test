use rstest::rstest;

fn a_plus_b(a: usize, b: usize) {
    println!("{}", a + b);
}

fn main() {
    println!("Hello World");
}
#[cfg(test)]
mod tests {
    use super::*;
    // with cases
    #[rstest]
    #[case(0, 2)]
    #[case(0, 3)]
    #[case(1, 2)]
    #[case(1, 3)]
    fn test_case(#[case] a: usize, #[case] b: usize) {
        a_plus_b(a, b);
    }

    // with values
    #[rstest]
    fn test_values(#[values(0, 1)] a: usize, #[values(2, 3)] b: usize) {
        a_plus_b(a, b);
    }

    // manual
    #[test]
    fn test_manual_1() {
        a_plus_b(0, 2);
    }

    #[test]
    fn test_manual_2() {
        a_plus_b(0, 3);
    }

    #[test]
    fn test_manual_3() {
        a_plus_b(1, 2);
    }

    #[test]
    fn test_manual_4() {
        a_plus_b(1, 3);
    }
}
