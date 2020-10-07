use crate::basic;
extern crate nalgebra;
use nalgebra::base::Vector2;
extern crate science;
use science::physics;

pub struct Entity{
    pub position: basic::Position,
    pub rotation: f64,
    pub mass: f64,
    pub motion: Vector2<f64>
}

impl Entity{
    pub fn gravitation_vector(&self, planet: Planet) -> Vector2{
        let a = self.position.x - planet.entity.position.x;
        let b = self.position.y - planet.entity.position.y;
        let c = (a.powi(2) + b.powi(2)).sqrt();
        // let theta = (a / c).acos();
        let force_g = (self.mass * planet.entity.mass) / c.powi(2);
        dx = force_g * a / self.mass;
        dy = force_g * b / self.mass;
        Vector2::new(dx, dy)
    }

    pub fn add_force(&self, force: physics::Force){
        let magnitude = force.magnitude / self.mass;
        dx = magnitude * cos(force.direction);
        dy = magnitude * sin(force.direction);
        let new_vec = Vector2::new(dx, dy);
        self.motion += new_vec;
    }
}

pub struct HitBox{
    width: f64,
    height: f64,
}

impl HitBox{
    
}

pub struct Planet{
    entity: Entity,
    cubic_position: basic::CubicCoordinate,
}

pub struct Player{
    entity: Entity,
}
