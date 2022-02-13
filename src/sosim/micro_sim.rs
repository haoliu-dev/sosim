/// a group of local people that has the same social traits in a habitant
struct Crowd {
  /// total people
  count: u32,
  /// higher educational level enables higher skill
  education_year: u32,
  
  
  
}

/// a legal entity formed by a group of individuals to engage in and
/// operate a business—commercial or industrial—enterprise.
/// ref: https://www.investopedia.com/terms/c/company.asp
/// # Productivity
/// productivity defines a company's products quality and quantity in
/// one simulation step.
struct Company {
  /// the type of business this company is doing
  business: String,

  // productivity
  /// technology level, defines what level of product it may make
  level: u32,
  // finance
  
}