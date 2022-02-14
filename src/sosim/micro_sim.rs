use serde::{Deserialize, Serialize};
use serde_json::Result;

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

/// This is a special kind of products only useful to `Company`, which
/// may acquire production output by buying `Capacity` from an `Equipment
/// Producer` or `Service Company`.
struct Capacity {
  /// the level of products produced and worker skill required
  level: u32,
  /// unit of product it may produce, can be negative
  /// a law or requirement may reduce the capacity
  output: f32,
  /// worker required, can be negative, `Service` may reduce the
  /// total worker required
  worker: f32,
  /// when(which step) it expires
  expiration: u32,
}

enum Product {
  Capacity(Capacity),
  Cad,
}


/// a legal entity formed by a group of individuals to engage in and
/// operate a business—commercial or industrial—enterprise.
/// ref: https://www.investopedia.com/terms/c/company.asp
/// # Productivity
/// productivity defines a company's products quality and quantity in
/// one simulation step. A certain level company may only produce that level
/// products, the output(products quantity) in one step is calculated as:
/// ```
/// output = equipment_productivity * equipment * worker
/// ```
/// ## Upgrade
/// a compnay productivity is at a certain level at a time, upgrade means
/// selling low-level equipment while purchase higher-level equipment
struct Company {
  /// the type of business this company is doing
  business: String,

  level: u32,
  /// how many products it may produce given sufficient input and worker.
  /// investment to equipment, services, infrastructure may increase capacity
  capacity: Vec<Capacity>,
}
