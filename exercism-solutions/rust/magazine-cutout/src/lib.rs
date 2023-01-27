// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_words: HashMap<&str, u32> = HashMap::new();
    for word in magazine {
        *magazine_words.entry(word).or_insert(0) += 1;
    }
    note.iter().all(|word| match magazine_words.get_mut(word) {
        Some(cnt) => {
            let enough_words = *cnt != 0;
            *cnt = (*cnt).saturating_sub(1);
            enough_words
        }
        None => false,
    })
}

// pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
//     let mut magazine_words: HashMap<&str, u32> =
//         magazine.iter().fold(HashMap::new(), |mut words, word| {
//             *words.entry(word).or_insert(0) += 1;
//             words
//         });

//     note.iter().all(|word| match magazine_words.get_mut(word) {
//         Some(cnt) => {
//             let enough_words = *cnt != 0;
//             *cnt = (*cnt).saturating_sub(1);
//             enough_words
//         }
//         None => false,
//     })
// }
