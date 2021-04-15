use std::collections::HashMap;

const CORRECT_NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    return if CORRECT_NUCLEOTIDES.contains(&nucleotide) {
        let mut result = 0;
        for n in dna.chars() {
            if CORRECT_NUCLEOTIDES.contains(&n) {
                if n == nucleotide {
                    result += 1;
                }
            } else {
                return Err(n);
            }
        }
        Ok(result)
    } else {
        Err(nucleotide)
    };
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut result: HashMap<char, usize> = HashMap::new();
    for n in &CORRECT_NUCLEOTIDES {
        result.insert(*n, 0);
    }
    for n in dna.chars() {
        if CORRECT_NUCLEOTIDES.contains(&n) {
            result.insert(n, result.get(&n).map(|x| x + 1).unwrap());
        } else {
            return Err(n);
        }
    }
    return Ok(result);
}
