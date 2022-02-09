// this module models society from macro economic view
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

enum IndustryTypeV2 {
  // 1st; output(material) = land * output
  Material, // Agriculture, Mining and Material
  // 2nd; input(land,material,human), output(goods)
  Equipment,     // Equipment Production
  ConsumerGoods, // Durable, NonDurable, food, beverage
  // 3rd; input(land,human), output(service)
  Infrastructure,  // land improvements, power, gas, water, communication
  BusinessService, // Financial/Information/...
  ConsumerService,
  // PublicService,
  HealthCare,
  Education,
  Entertainment,
}

enum ScienceType {
  Physics,
  Chemistry,
  Biology,
  Environment,
  Admin,
  Arts,
  Economy,
}

#[derive(Serialize, Deserialize, Debug)]
enum IndustryKind {
  NaturalResource = 1, // 1st
  Factory,             // 2nd
  Service,             // 3rd
}

#[derive(Serialize, Deserialize, Debug)]
struct Industry {
  name: String,
  kind: IndustryKind,
}
// country is the whole model of the macro economy of player's context
#[derive(Serialize, Deserialize, Debug)]
pub struct Society {
  industries: Vec<Industry>,
}

impl Society {
  pub fn loadFromJson(json: &str) -> Result<Society> {
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
