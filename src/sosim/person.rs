use rand::prelude::*;
use rand_distr::StandardNormal;

#[derive(Debug, Clone, Copy)]
pub struct Person {
  // born-characteristics: init as standard normalized randoms
  // aggressive: f32,   // how aggressive in making life choice
  // compliance: f32, // comformity to standards, conventions, laws
  // resiliency: f32, // how resilient and firmness towards goals
  intelligence: f32, // how quick skill may grow
  strength: f32,     // the quality of being strong
  // status: init=0 or inherited,
  // updated by step() with function(personal-char, environment)
  happiness: f32,
  wealth: f32,         // net asset
  prestige: f32,       // social prestige
  age: u32,            // cycle
  political_bias: f32, // political spectrum: left(-) or right(+)
  skill: f32,          // increased by education and work
}

impl Person {
  pub fn new(r: &mut ThreadRng, inherited_wealth: f32, inherited_prestige: f32) -> Person {
    Person {
      // aggressive: r.sample::<f32, _>(StandardNormal),
      // compliance: r.sample::<f32, _>(StandardNormal),
      intelligence: r.sample::<f32, _>(StandardNormal).abs(),
      strength: r.sample::<f32, _>(StandardNormal).abs(),
      age: 0,
      happiness: 0.0,
      wealth: (1.0 + r.sample::<f32, _>(StandardNormal)) * inherited_wealth,
      prestige: (1.0 + r.sample::<f32, _>(StandardNormal)) * inherited_prestige,
      political_bias: 0.0,
      skill: 0.0,
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
    // environments
    // effective interest rate + 100%
    let interest_rate = (1.0 + r.sample::<f32, _>(StandardNormal) / 2.0).abs();
    let base_spending = 1.9;
    let work_efficiency = 1.0;
    // personal stats
    let wage = work_efficiency * (1.0 + self.skill).abs();
    let spending = base_spending + (self.wealth * 0.05 * r.sample::<f32, _>(StandardNormal)).abs();

    // age ++
    self.age += 1;
    // wealth
    let new_wealth = self.wealth * interest_rate + wage - spending;
    // personal growth
    self.happiness += new_wealth / self.wealth;
    let new_skill = self.skill + (r.sample::<f32, _>(StandardNormal) + self.intelligence);
    self.skill = new_skill.abs();
    // update
    self.wealth = new_wealth;
  }
}
