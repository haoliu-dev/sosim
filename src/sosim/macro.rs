// this module models society from macro economic view

// industry defined in https://zh.wikipedia.org/wiki/%E4%B8%AD%E5%9B%BD%E4%BA%A7%E4%B8%9A%E5%88%86%E7%B1%BB
enum IndustryType {
  // 1st
  Agriculture,  // 农林牧渔
  // 2nd business facing
  Mining,  // 采矿，石油
  Equipment, // 装备制造业
  Material, //材料，金属加工
  // 2nd consumer goods https://www.investopedia.com/terms/c/consumer-goods.asp
  Durable, // 耐用品:汽车，家具, 电子产品, preserve value in long term
  NonDurable, //非耐用品：日化; FoodBeverage, // 食品饮料; Pharma, //制药
  // 2nd both facing
  Infrastructure, // power, water, gas
  Construction,
  // 3rd
  BusinessService,
  ConsumerService, // daily service for consumers
  Sales, // wholesale + retail
  Transportation,
  HotelRestaurant, // hotel and restaurant
  It, // IT, broadcasting, communication
  FinancialService,
  Education,
  Entertainment,
  PublicService, // Government, self-govern

}