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

#[derive(Serialize, Deserialize, Debug)]
enum IndustryKind {
  NaturalResource = 1, // 1st
  Production,             // 2nd
  Service,             // 3rd
}

/// This represents a whole Industry Department in the sim
/// # a `Industry` has one or more upstream `Industry`
#[derive(Serialize, Deserialize, Debug)]
struct Industry {
  /// globally unique name of the industry
  name: String,
  /// 1st(natural resource), 2nd(production) or 3rd(service) industry department
  kind: IndustryKind,
  // services that may boost production of this industry(exclude equipment industry)
  boosters: Vec<String>,
  // production capacity(Vec value, measured in output product units per cycle) of each level(Vec index)
  // each type of products has its minimum level requirements,
  // eg. Car.min_level = 3; and capacity from idx=3 may devote to car production
  // higher level may produce more advanced car(more added value)
  capacity: Vec<f32>,
  // 
  // production added value(Vec value) per 1 unit of upstream material for each level(Vec index)
  added_value: Vec<f32>,
  // scale of this industry
  // automation ratio(1~INF): scale up efficiency,
  // buy more and higher level equipments increase automation more
  // the higher automation, the higher skill needed(entry bar) for workers
  automation: f32, // default: 1.0f (no automation)
  // capacity: f32, // production capacity assuming unrestricted input
  // FIXME: use a better model for equipment scaled efficiency and capacity
}
#[derive(Serialize, Deserialize, Debug)]
struct Product {
  name: String,
  industry_chain: Vec<Industry>,
}
// country is the whole model of the macro economy of player's context
#[derive(Serialize, Deserialize, Debug)]
pub struct Society {
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
