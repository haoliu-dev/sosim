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

/// defines a level in one industries
#[derive(Serialize, Deserialize, Debug)]
struct IndustryLevel {
  /// minimum cycle needed to upgrade to next level for both industry and worker
  ///
  /// 1. worker upgrade rules
  /// worker's working cycle required to upgrade to next level.
  /// once a worker has worked `upgrade_cycle` cycles, the worker may upgrade
  /// to next level with certain chance
  ///
  /// 2. industry upgrade rules
  /// industry running for `upgrade_cycle` cycles on current max level may
  /// upgrade to next level with certain chance
  // upgrade_cycle: u16,

  /// how many `research_unit` needed to upgrade to next level
  /// a `research_unit` is defined as one same-level worker dedicate to
  /// research in one cycle. Hire more researcher speeds the upgrade.
  research: f32,

  /// one worker of the same level may produce how many units of products in one cycle
  /// this is used to determine how many workers are required for certain level and industry
  /// capacity.
  ///
  /// a industry may produce `productivity * worker_count` units of products
  /// (up to capacity for that level) for an given number in one industry
  worker_productivity: f32,

  /// one unit of same-level equipment may produce how many units of products in one cycle
  /// `worker_productivity` and `equipment_productivity` defines the capital intensity
  /// for this industry
  equipment_productivity: f32,
}

/// This represents a whole Industry Department in the sim
#[derive(Serialize, Deserialize, Debug)]
struct Industry {
  /// globally unique name of the industry
  name: String,

  //--- characteristics ---//
  /// products it makes
  products: Vec<Product>,

  capacities: Vec<Productivity>,
  // settings
}

/// This is a special kind of products only useful to `Industry`, which
/// may acquire production output by buying `Productivity` from an `Equipment
/// Producer` or `Service Industry`.
/// https://en.wikipedia.org/wiki/Productivity
#[derive(Serialize, Deserialize, Debug)]
struct Productivity {
  /// the level of products produced and worker skill required
  level: u32,
  /// productivity(entropy reduction) in one step, can be negative
  /// a law or requirement may reduce the capacity
  output: f32,
  /// worker required, can be negative, `Service` may reduce the
  /// total worker required
  worker: f32,
  /// when(which step) it expires
  expiration: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct ProductRecipe {
  name: String,
  unit: f32,
}
#[derive(Serialize, Deserialize, Debug)]
struct Product {
  /// unique name for this product
  name: String,

  /// product level, also is the minimum required industry level to produce this
  level: u8,

  /// how many input product units needed to produce one unit of this product
  /// if no material needed(like natural resource or service provider),
  /// leave it empty
  recipe: Vec<ProductRecipe>,

  /// how much unit of it does one consumer need for each cycle
  /// 0.0f means no consumer needs, it's a material or equipment for production
  consumer_needs: f32,
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
