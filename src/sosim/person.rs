use rand::prelude::*;
use rand_distr::StandardNormal;

#[derive(Debug, Clone, Copy)]
pub struct Person {
  // born-characteristics: init as standard normalized randoms
  // aggressive: f32,   // how aggressive in making life choice
  // compliance: f32, // comformity to standards, conventions, laws
  intelligence: f32, // how quick skill may grow
  strength: f32,     // the quality of being strong
  // status: init=0 or inherited,
  // updated by step() with function(personal-char, environment)
  happiness: f32,
  wealth: f32,         // net asset
  prestige: f32,       // social prestige
  age: u32,            // cycle
  political_bias: f32, // political spectrum: left(-) or right(+)
  skill: u8,           // increased by education and work
}

impl Person {
  pub fn new(r: &mut ThreadRng, inherited_wealth: f32, inherited_prestige: f32) -> Person {
    Person {
      // aggressive: r.sample::<f32, _>(StandardNormal),
      // compliance: r.sample::<f32, _>(StandardNormal),
      intelligence: r.sample::<f32, _>(StandardNormal),
      strength: r.sample::<f32, _>(StandardNormal),
      age: 0,
      happiness: 0.0,
      wealth: (1.0 + r.sample::<f32, _>(StandardNormal)) * inherited_wealth,
      prestige: (1.0 + r.sample::<f32, _>(StandardNormal)) * inherited_prestige,
      political_bias: 0.0,
      skill: 0,
    }
  }

  // step n cycles
  pub fn step(&mut self, r: &mut ThreadRng, n: u32) {
    for _i in 0..n {
      self.step_one(r);
    }
  }

  // simulate one cycle
  fn step_one(&mut self, r: &mut ThreadRng) {
    // age ++
    self.age += 1;
  }
}
