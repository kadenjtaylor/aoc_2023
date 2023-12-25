mod domain;

#[test]
fn run_example() {
    use domain::{closest_location, mappings, Almanac};
    let test_almanac = Almanac {
        seeds: vec![79, 14, 55, 13],
        seed_to_soil: mappings(vec![(50, 98, 2), (52, 50, 48)]),
        soil_to_fertilizer: mappings(vec![(0, 15, 37), (37, 52, 2), (39, 0, 15)]),
        fertilizer_to_water: mappings(vec![(49, 53, 8), (0, 11, 42), (42, 0, 7), (57, 7, 4)]),
        water_to_light: mappings(vec![(88, 18, 7), (18, 25, 70)]),
        light_to_temperature: mappings(vec![(45, 77, 23), (81, 45, 19), (68, 64, 13)]),
        temperature_to_humidity: mappings(vec![(0, 69, 1), (1, 0, 69)]),
        humidity_to_location: mappings(vec![(60, 56, 37), (56, 93, 4)]),
    };
    let loc = closest_location(&test_almanac);
    assert_eq!(loc, Some(35));
}

pub fn run() {
    let use_seed_ranges = true;
    let almanac = domain::retrieve_from_file(use_seed_ranges);
    println!("Seeds collected: {}", almanac.seeds.len());
    let loc = domain::closest_location(&almanac);
    println!("Closest location is: {:?}", loc);
}
