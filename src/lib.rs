pub mod about_asserts;
pub mod about_true_and_false;
pub mod about_type_str;

use std::any::type_name;

pub fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

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

    #[test]
    fn test_type_of() {
        assert_eq!("&str", type_of("string"));
    }
}
