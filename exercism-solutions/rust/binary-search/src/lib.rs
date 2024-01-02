// Boring implementation
// pub fn find(array: &[i32], key: i32) -> Option<usize> {
//     if array.is_empty() {
//         return None;
//     }

//     let mut left: i64 = 0;
//     let mut right: i64 = (array.len() - 1) as i64;

//     while left <= right {
//         let mid = (left + right) / 2;
//         let value = array[mid as usize];

//         if value == key {
//             return Some(mid as usize);
//         }

//         if value < key {
//             left = mid + 1;
//         } else if value > key {
//             right = mid - 1;
//         }
//     }

//     None
// }

// Boring implementation
pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = array.len();

    while left < right {
        let mid = (left + right) / 2;
        let value = array[mid];

        if value == key {
            return Some(mid);
        }

        if value < key {
            left = mid + 1;
        } else if value > key {
            right = mid;
        }
    }

    None
}
