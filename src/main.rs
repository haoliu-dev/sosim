mod sosim;


fn main() {
    println!("====Social simulation====");
    let init_config = sosim::InitConfig {person_count: 3};
    let country = sosim::init(init_config);
    let status = country.step(1);
    println!("{}\n", status);
}
