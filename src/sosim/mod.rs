use rand::prelude::*;
use std::io;
use std::io::prelude::*;
use std::fs::File;


mod person;
use person::Person;

mod macro_sim;
use macro_sim::Society;

// sim the country with macro economy simulator
pub fn start_macro_sim() -> io::Result<()>{
  let mut f = File::open("config/society.json")?;
  let mut buffer = String::new();

  f.read_to_string(&mut buffer)?;
  let r = Society::load_json(&buffer);
  match r {
    Ok(s) => {
      println!("{:#?}", s)
    }
    Err(e) => {
      println!("***Error: {}", e)
    }
  }
  Ok(())
}

pub struct InitConfig {
  pub person_count: usize,
}
pub fn init(config: InitConfig) -> Country {
  let mut r = thread_rng(); // get a clone of thread_rng() and reuse
  let inherited_wealth = 20.0;
  let inherited_prestige = 0.0;
  let country = Country {
    age: 0,
    people: (0..config.person_count)
      .map(|_x| Person::new(&mut r, inherited_wealth, inherited_prestige))
      .collect(),
  };
  return country;
}

pub struct Country {
  age: u32,
  people: Vec<Person>,
}
impl Country {
  pub fn step(&mut self, n: u32) -> String {
    let mut r = thread_rng(); // get a clone of thread_rng() and reuse
    self.age += n;
    let mut ret = String::new();
    ret.push_str(&format!(
      "Country[age={},people={}]\n",
      self.age,
      self.people.len()
    ));
    for p in &mut self.people {
      p.step(&mut r, n);
      ret.push_str(&format!("{:?}\n", p))
    }
    return ret;
  }
}
