use std::cmp::Ordering;
/// binary search a ordered list, returns the index of the key item
pub fn binary_search<T: Ord>(array: &[T], key: T) -> Option<usize> {
    let len = array.len();
    
    if len == 0 {
        None
    } else {
        let mid = len / 2;
        match &array[mid].cmp(&key) {
            Ordering::Equal => Some(mid),
            Ordering::Greater => binary_search(&array[..mid], key),
            Ordering::Less => binary_search(&array[mid + 1..], key),
        }
    }
}

#[cfg(test)]
mod test_binary_search {
    use super::binary_search;

    #[test]
    fn basic() {
        let array = [1, 2, 3];
        assert_eq!(binary_search(&array, 2).unwrap(), 1);
    }
    #[test]
    fn not_include() {
        let array = [1, 2, 3];
        assert_eq!(binary_search(&array, 4), None);
    }
    #[test]
    fn empty() {
        assert_eq!(binary_search(&[], 1), None);
    }

    #[test]
    fn vector_type() {
        assert_eq!(binary_search(&vec![1, 2, 3], 2), Some(1));
    }
    #[test]
    fn works_for_structs() {
        #[derive(PartialEq, Eq, PartialOrd, Ord)]
        struct Person {
            id: u32,
            name: &'static str,
        }
        let array = [
            Person { id: 0, name: "jay" },
            Person {
                id: 1,
                name: "mary",
            },
            Person {
                id: 5,
                name: "john",
            },
        ];
        assert_eq!(
            binary_search(
                &array,
                Person {
                    id: 1,
                    name: "mary"
                }
            ),
            Some(1)
        );
    }
}
