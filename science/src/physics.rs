extern crate nalgebra;

const HEXAGON_VOLUME: f64 = 1.082531755; //Liters
const UGC: f64 = 8.3145; //Universal Gas Constant R

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
