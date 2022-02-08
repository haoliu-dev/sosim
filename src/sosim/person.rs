use rand::prelude::*;
use rand_distr::StandardNormal;


#[derive(Debug, Clone, Copy)]
pub struct Person {
  // attributes: impacting step calculation, randomly generated/updated
  base_consumption: f32,
  base_production: f32,
  // status: updated by step calculation according to formula
  health: f32,
  happiness: f32,

  wealth: f32, // net asset
  age: f32,
  // skill
  production_skill_level: f32,
}

impl Person {
  pub fn new() -> Person {
    let mut r = thread_rng();
    Person {
      base_consumption: r.sample::<f32,_>(StandardNormal),
      base_production: r.sample::<f32,_>(StandardNormal),
      age: 1.0,
      health: r.sample::<f32,_>(StandardNormal),
      happiness: r.sample::<f32,_>(StandardNormal),
      wealth: r.sample::<f32,_>(StandardNormal) * 100.0,
      production_skill_level: 1.0,
    }
  }

  fn step(n: usize) {

  }

  fn consumption(self)-> f32 {
    self.base_consumption * self.age
  }
}
