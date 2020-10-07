extern crate nalgebra;

const HEXAGON_VOLUME: f64 = 1.082531755; //Liters
const UGC: f64 = 8.3145; //Universal Gas Constant R
const GRAV: f64 = 6.67408e-11;

pub struct Force{
    direction: f64,//theta
    magnitude: f64,//Newtons
}

pub struct PhysicalProperties{
    pub min_grain_size: f32,
    pub max_grain_size: f32,
    pub grain_size_function: u8,
    pub min_crystal_size: f32,
    pub max_crystal_size: f32,
    pub crystal_size_function: u8,
    pub crystal_shape: u8,
}

impl PhysicalProperties{
    pub fn new() -> PhysicalProperties{
        PhysicalProperties{
            min_grain_size: 0.0,
            max_grain_size: 0.0,
            grain_size_function: 0,
            min_crystal_size: 0.0,
            max_crystal_size: 0.0,
            crystal_size_function: 0,
            crystal_shape: 0,
        }
    }
}

//Motion Functions



//Gas function PV=nRT R=8.3145(L*kPa/K*mol)
pub fn get_pressure(mols: f64, kelvin: f64) -> f64{
    return (mols* UGC * kelvin) / HEXAGON_VOLUME;
}

//mass in kilograms
pub fn get_mass(mols: f64, molar_mass: f64) -> f64{
    (mols * molar_mass) / 1000.0
}

//Density = mass/Volume
pub fn get_density(mass: f64) -> f64{
    mass / HEXAGON_VOLUME
}
