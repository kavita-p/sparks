use std::collections::HashSet;
use std::hash::Hash;

pub fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

#[cfg(test)]
mod lib_tests {
    use super::*;

    #[test]
    fn check_doubles() {
        let vec_with_doubles = vec![1, 2, 4, 4];

        assert!(!has_unique_elements(vec_with_doubles));
    }

    #[test]
    fn check_no_doubles() {
        let vec_without_doubles = vec![1, 2, 3, 4];

        assert!(has_unique_elements(vec_without_doubles));
    }
}
