// Boring (but somewhat clean) implementation
// pub fn find(array: &[i32], key: i32) -> Option<usize> {
//     let mut left = 0;
//     let mut right = array.len();

//     while left < right {
//         let mid = (left + right) / 2;
//         let value = array[mid];

//         if value == key {
//             return Some(mid);
//         }

//         if value < key {
//             left = mid + 1;
//         } else if value > key {
//             right = mid;
//         }
//     }

//     None
// }

use std::cmp::Ordering;

// Not so boring implementation
pub fn find<KeyType, CollectionType>(array: CollectionType, key: KeyType) -> Option<usize>
where
    KeyType: Ord,
    CollectionType: AsRef<[KeyType]>,
{
    let array = array.as_ref();
    let mut left = 0;
    let mut right = array.len();

    while left < right {
        let mid = (left + right) / 2;
        match array[mid].cmp(&key) {
            Ordering::Equal => return Some(mid),
            Ordering::Less => left = mid + 1,
            Ordering::Greater => right = mid,
        };
    }

    None
}
