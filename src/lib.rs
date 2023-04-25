pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn divide(left: usize, right: usize) -> usize {
    left / right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn divide_works() {
        let result = divide(2, 2);
        assert_eq!(result, 1);
    }
}
