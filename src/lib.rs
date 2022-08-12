/// Adds two numbers given.
/// 与えられた2つの数値を足す。
///
/// # Examples
///
/// ```
/// assert_eq!(3, my_crate::add(1, 2));
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
