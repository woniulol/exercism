pub struct Allergies {
    score: u32,
    _allergen_flag: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

use Allergen::*;
use std::iter::zip;

const ALLERGEN_SEQUENCE: [Allergen; 8] = [
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
];

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self {
            score,
            _allergen_flag: (&format!("{:b}", score))
                .chars()
                .map(|c| (c as u8) - b'0')
                .collect(),
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let index = ALLERGEN_SEQUENCE
            .iter()
            .position(|a| a == allergen)
            .unwrap();
        if self._allergen_flag.len() > index {
            return *(self
                ._allergen_flag
                .iter()
                .rev()
                .skip(if index > 0 { index } else { 0 })
                .next()
                .unwrap())
                == 1 as u8;
        } else {
            false
        }
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut res: Vec<Allergen> = Vec::new();
        let mut iter = zip(self._allergen_flag.iter().rev(), ALLERGEN_SEQUENCE.iter());

        while let Some((&i, &a)) = iter.next() {
            if i == 1 {
                res.push(a);
            }
        }
        res
    }
}
