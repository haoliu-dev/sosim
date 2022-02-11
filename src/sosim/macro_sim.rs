/// this module models society from macro economic view
use serde::{Deserialize, Serialize};
use serde_json::Result;

// industry defined in https://zh.wikipedia.org/wiki/%E4%B8%AD%E5%9B%BD%E4%BA%A7%E4%B8%9A%E5%88%86%E7%B1%BB
enum ComprehensiveIndustryType {
  // 1st
  Agriculture, // 农林牧渔
  // 2nd business facing
  Mining,    // 采矿，石油
  Equipment, // 装备制造业
  Material,  //材料，金属加工
  // 2nd consumer goods https://www.investopedia.com/terms/c/consumer-goods.asp
  Durable,    // 耐用品:汽车，家具, 电子产品, preserve value in long term
  NonDurable, //非耐用品：日化; FoodBeverage, // 食品饮料; Pharma, //制药
  // 2nd both facing
  Infrastructure, // power, water, gas
  Construction,
  // 3rd
  BusinessService,
  ConsumerService, // daily service for consumers
  Sales,           // wholesale + retail
  Transportation,
  HotelRestaurant, // hotel and restaurant
  It,              // IT, broadcasting, communication
  FinancialService,
  Education,
  HealthCare,
  Entertainment,
  PublicService, // Government, self-govern
}

/// Define one science in the sim
#[derive(Serialize, Deserialize, Debug)]
struct Science {
  name: String,
  experience: f32, // experience accumulated, higher the experience, higher chance to level up
  level: u8,
}

/// defines a uniform level in all industries and workers
/// level ordinal is defined by its position in `Society.Levels` array
#[derive(Serialize, Deserialize, Debug)]
struct Level {
  /// a human-readable name for that level
  /// level ordinal is defined by its position in `Society.Levels` array
  name: String,

  /// minimum cycle needed to upgrade to next level for both industry and worker
  ///
  /// 1. worker upgrade rules
  /// worker's working cycle required to upgrade to next level.
  /// once a worker has worked `upgrade_cycle` cycles, the worker may upgrade
  /// to next level with certain chance, which gets higher with `worker.intelligence`
  ///
  /// 2. industry upgrade rules
  /// industry running for `upgrade_cycle` cycles on current max level may
  /// upgrade to next level with certain chance, which gets higher with
  /// `industry.research`
  upgrade_cycle: u16,

  /// one worker of the same level may produce how many units of products in one cycle
  /// this is used to determine how many workers are required for certain level and industry
  /// capacity.
  ///
  /// a industry may produce `productivity * worker_count` units of products
  /// (up to capacity for that level) for an given number in one industry
  productivity: f32,
}

#[derive(Serialize, Deserialize, Debug)]
enum IndustryKind {
  NaturalResource = 1, // 1st
  Production,          // 2nd
  Service,             // 3rd
}

/// This represents a whole Industry Department in the sim
#[derive(Serialize, Deserialize, Debug)]
struct Industry {
  /// globally unique name of the industry
  name: String,
  /// 1st(natural resource), 2nd(production) or 3rd(service) industry department
  kind: IndustryKind,

  /// `<status>` production capacity of each level in this industry,
  /// here's how it works:
  /// 1. capacity(array) defines the maximum production unit per cycle(array element value)
  ///  for that level(array element index)
  /// 2. you add capacity of a certain level by adding the `equipment product` of
  /// that level, that `product.is_equipment_for (Vec<String>)` must include
  /// the name of this industry
  /// 3. you may only produce up to the capacity given sufficient worker and input material
  ///
  /// For example, the following capacity means currently,
  /// this industry has production capacity
  /// of three levels(0,1,2), the production capacity for the
  /// levels are 150, 20, 12 unit per cycle, respectively
  /// ```
  /// capacity = [150.0, 30.0, 12.0]
  /// ```
  capacity: Vec<f32>,

  /// accumulated research spending, the higher, the higher chance
  /// to upgrade to next level once `level.upgrade_cycle` reached
  research: f32,
}
#[derive(Serialize, Deserialize, Debug)]
struct Product {
  name: String,
  industry_chain: Vec<Industry>,
}
// country is the whole model of the macro economy of player's context
#[derive(Serialize, Deserialize, Debug)]
pub struct Society {
  levels: Vec<Level>,
  industries: Vec<Industry>,
  sciences: Vec<Science>,
}

impl Society {
  // various constructor
  pub fn load_json(json: &str) -> Result<Society> {
    let r: Result<Society> = serde_json::from_str(json);
    return r;
  }
}

struct IndustryDepartment {
  kind: IndustryKind,
  // efficiency related
  // invested equipment boost efficiency in log(x) level
  // invested equipment depreciates by certain rate per cycle
  equipment_invested: f32,
  equipment_depreciation_rate: f32, // 0~1
  // status
  capacity: f32, // production capacity
  employee: f32, // employee count
}

// impl IndustryDepartment {
//   // calculate output per cycle(given unrestricted input) for this industry:
//   // worker.output = worker.basic_output * worker.skill * industry.efficiency
//   fn efficiency(self, &industry_catalog:IndustryCatalog) -> f32 {
//     match self.kind {
//       IndustryType::Material => {
//         self.equipment_invested * industry_catalog.getIndustry(IndustryType::Infrastructure)
//       },
//       IndustryType::Factory => {
//         self.efficiency
//       },
//       IndustryType::Service => {

//       }
//     }
//   }
// }
