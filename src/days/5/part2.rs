mod domain;

pub fn run() {
    let use_seed_ranges = true;
    let almanac = domain::retrieve_from_file(use_seed_ranges);
    println!("Seeds collected: {}", almanac.seeds.len());
    let loc = domain::closest_location(&almanac);
    println!("Closest location is: {:?}", loc);
}
