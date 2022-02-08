mod person;
use person::Person;

pub struct InitConfig {
  pub person_count: usize,
}
pub fn init(config: InitConfig) -> Country {
  let person_prototype = Person::new();
  let country = Country {
    people: vec![person_prototype; config.person_count],
  };
  return country;
}

pub struct Country {
  people: Vec<Person>,
}
impl Country {
  pub fn step(self, _n: usize) -> String {
    let mut ret = String::new();
    ret.push_str(&format!(
      "In this country:\nperson_count: {}\n",
      self.people.len()
    ));
    for p in self.people {
      ret.push_str(&format!("{:?}\n",p))
    }
    return ret;
  }
}
