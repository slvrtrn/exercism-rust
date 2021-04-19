use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref DNA_TO_RNA: HashMap<char, char> = {
        let mut m = HashMap::new();
        m.insert('G', 'C');
        m.insert('C', 'G');
        m.insert('A', 'T');
        m.insert('U', 'A');
        m
    };
    static ref RNA_TO_DNA: HashMap<char, char> = {
        let mut m = HashMap::new();
        for (k, v) in DNA_TO_RNA.iter() {
            m.insert(*v, *k);
        }
        m
    };
}

#[derive(Debug, PartialEq)]
pub struct Dna {
    seq: String
}

#[derive(Debug, PartialEq)]
pub struct Rna {
    seq: String
}

fn construct(dict: &HashMap<char, char>, input: &str) -> Result<String, usize> {
    let mut seq = Vec::with_capacity(input.len());
    for (i, c) in input.chars().enumerate() {
        match dict.get(&c) {
            Some(x) => seq.push(*x),
            None => return Err(i),
        }
    }
    return Ok(seq.iter().collect());
}

impl Dna {
    pub fn new(rna: &str) -> Result<Dna, usize> {
        construct(&RNA_TO_DNA, rna)
            .map(|seq| Self { seq })
    }

    pub fn into_rna(self) -> Rna {
        Rna::new(self.seq.as_str()).unwrap()
    }
}

impl Rna {
    pub fn new(dna: &str) -> Result<Rna, usize> {
        construct(&DNA_TO_RNA, dna)
            .map(|seq| Self { seq })
    }
}
