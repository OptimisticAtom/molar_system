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

pub struct Material{
    pub color: [f32; 4],
    pub default_solid_state: i8,
    pub melting_point: f64,
    pub boiling_point: f64,
    pub molar_mass: f64,
}

pub struct MaterialState{
    pub kelvin: f64,
    pub mols: f64,
    pub state_of_matter: u8,
}

pub struct MaterialDictionary{
    pub dictionary: HashMap<String, Material>,
}

impl MaterialDictionary {
    pub fn new() -> MaterialDictionary{
        let mut hashmap = HashMap::new();
        hashmap.insert("silica".to_string(), Material{
            color: [0.9, 1.0, 0.9, 1.0],
            default_solid_state: 0,
            melting_point: 1986.0,
            boiling_point: 3220.0,
            molar_mass: 60.084,

        });
        hashmap.insert("air".to_string(), Material{
            color: [0.1, 0.5, 0.9, 0.7],
            default_solid_state: 0,
            melting_point: 63.15,
            boiling_point: 77.355,
            molar_mass: 28.014,
        });
        MaterialDictionary{dictionary: hashmap}
    }

    pub fn access_dictionary(&self, formula: &String) -> &Material{
        self.dictionary.get(formula).unwrap()
    }
}
