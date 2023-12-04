use std::collections::HashMap;

const VALID_NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

fn is_valid(ch: char) -> bool {
    VALID_NUCLEOTIDES.contains(&ch)
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !is_valid(nucleotide) {
        return Err(nucleotide);
    }

    dna.chars().try_fold(0, |acc, ch| {
        if !is_valid(ch) {
            return Err(ch);
        }

        if ch == nucleotide {
            return Ok(acc + 1);
        }

        Ok(acc)
    })
}

pub fn count_functional(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !is_valid(nucleotide) {
        return Err(nucleotide);
    }

    dna.chars().try_fold(0, |acc: usize, ch: char| {
        Ok(if is_valid(ch).then_some(ch == nucleotide).ok_or(ch)? {
            acc + 1
        } else {
            acc
        })
    })
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    VALID_NUCLEOTIDES
        .iter()
        .try_fold(HashMap::new(), |mut map, nucl| {
            map.insert(*nucl, count(*nucl, dna)?);
            Ok(map)
        })
}
