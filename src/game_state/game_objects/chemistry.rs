use std::collections::HashMap;

pub struct Element{
    name: String,
    atomic_number: i8,
    atomic_mass: f64,
    electro_negativity: f64,
    valence_electrons: i8,
}

#[derive(Copy, Clone)]
pub struct Molecule{
    pub color: [f32; 4]
}

pub struct MaterialDictionary{
    pub dictionary: HashMap<String, Molecule>,
}

impl MaterialDictionary {
    pub fn new() -> MaterialDictionary{
        let mut hashmap = HashMap::new();
        hashmap.insert("stone".to_string(), Molecule{color: [0.5, 0.5, 0.5, 1.0]});
        hashmap.insert("dirt".to_string(), Molecule{color: [0.6, 0.6, 0.2, 1.0]});
        hashmap.insert("air".to_string(), Molecule{color: [0.1, 0.5, 0.9, 0.7]});
        MaterialDictionary{dictionary: hashmap}
    }

    pub fn access_dictionary(&self, formula: &String) -> Molecule{
        *self.dictionary.get(formula).unwrap()
    }
}
