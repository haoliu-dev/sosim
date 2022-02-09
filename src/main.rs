mod sosim;

fn main() {
    // sim_micro();
    sosim::start_macro_sim();
}

fn sim_micro() {
    println!("====Social simulation====");
    let init_config = sosim::InitConfig {person_count: 3};
    let mut country = sosim::init(init_config);
    let status = country.step(1);
    println!("{}\n", status);
    let status = country.step(100);
    println!("{}\n", status);
}