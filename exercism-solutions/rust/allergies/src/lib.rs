pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

// impl TryFrom<u32> for Allergen {
//     type Error = ();

//     fn try_from(score: u32) -> Result<Self, Self::Error> {
//         match score {
//             x if x == Allergen::Eggs as u32 => Ok(Allergen::Eggs),
//             x if x == Allergen::Peanuts as u32 => Ok(Allergen::Peanuts),
//             x if x == Allergen::Shellfish as u32 => Ok(Allergen::Shellfish),
//             x if x == Allergen::Strawberries as u32 => Ok(Allergen::Strawberries),
//             x if x == Allergen::Tomatoes as u32 => Ok(Allergen::Tomatoes),
//             x if x == Allergen::Chocolate as u32 => Ok(Allergen::Chocolate),
//             x if x == Allergen::Pollen as u32 => Ok(Allergen::Pollen),
//             x if x == Allergen::Cats as u32 => Ok(Allergen::Cats),
//             _ => Err(()),
//         }
//     }
// }

// impl Allergies {
//     pub fn new(score: u32) -> Self {
//         Self { score }
//     }

//     pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
//         let allergen_score = *allergen as u32;
//         self.score & allergen_score == allergen_score
//     }

//     pub fn allergies(&self) -> Vec<Allergen> {
//         let mut allergies = Vec::new();
//         for i in 0..8 {
//             let allergen = Allergen::try_from(1 << i);
//             if let Ok(allergen) = allergen {
//                 if self.is_allergic_to(&allergen) {
//                     allergies.push(allergen);
//                 }
//             }
//         }
//         allergies
//     }
// }

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let allergen_score = *allergen as u32;
        self.score & allergen_score != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        (0..8)
            .filter_map(|idx| {
                let allergen = unsafe { std::mem::transmute::<u8, Allergen>(1 << idx) };
                self.is_allergic_to(&allergen).then_some(allergen)
            })
            .collect()
    }
}
