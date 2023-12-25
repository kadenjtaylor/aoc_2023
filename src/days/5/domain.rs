use std::ops::Range;

#[derive(Debug)]
pub struct Mapping {
    source_range: Range<i64>,
    destination_range: Range<i64>,
}

fn mapping_from(dest: i64, source: i64, length: i64) -> Mapping {
    Mapping {
        source_range: (source..source + length),
        destination_range: (dest..dest + length),
    }
}

pub fn mappings(v: Vec<(i64, i64, i64)>) -> Vec<Mapping> {
    v.iter().map(|(d, s, l)| mapping_from(*d, *s, *l)).collect()
}

#[derive(Debug)]
pub struct Almanac {
    pub seeds: Vec<i64>,
    pub seed_to_soil: Vec<Mapping>,
    pub soil_to_fertilizer: Vec<Mapping>,
    pub fertilizer_to_water: Vec<Mapping>,
    pub water_to_light: Vec<Mapping>,
    pub light_to_temperature: Vec<Mapping>,
    pub temperature_to_humidity: Vec<Mapping>,
    pub humidity_to_location: Vec<Mapping>,
}

fn empty_almanac() -> Almanac {
    Almanac {
        seeds: vec![],
        seed_to_soil: vec![],
        soil_to_fertilizer: vec![],
        fertilizer_to_water: vec![],
        water_to_light: vec![],
        light_to_temperature: vec![],
        temperature_to_humidity: vec![],
        humidity_to_location: vec![],
    }
}

pub fn retrieve_from_file() -> Almanac {
    let mut alm = empty_almanac();
    let chunks = include_str!("../../../resources/almanac.txt").split("\n\n");
    chunks.enumerate().for_each(|(index, chunk)| {
        println!("{chunk}");
        if index == 0 {
            let mut chunklets = chunk.split(":");
            chunklets.next(); // skip "seeds:"
            if let Some(c) = chunklets.next() {
                c.lines().for_each(|line| {
                    line.split_whitespace()
                        .flat_map(|s| s.parse::<i64>())
                        .for_each(|n| alm.seeds.push(n))
                });
            }
        } else {
            let mappings = chunk
                .lines()
                .map(|line| line.split_ascii_whitespace().flat_map(|i| i.parse::<i64>()))
                .flat_map(|i| {
                    let vec = i.collect::<Vec<i64>>();
                    if vec.len() == 3 {
                        Some(mapping_from(vec[0], vec[1], vec[2]))
                    } else {
                        None
                    }
                })
                .collect::<Vec<Mapping>>();
            match index {
                1 => alm.seed_to_soil = mappings,
                2 => alm.soil_to_fertilizer = mappings,
                3 => alm.fertilizer_to_water = mappings,
                4 => alm.water_to_light = mappings,
                5 => alm.light_to_temperature = mappings,
                6 => alm.temperature_to_humidity = mappings,
                7 => alm.humidity_to_location = mappings,
                _ => println!("Not sure how this happened..."),
            };
        }
    });
    alm
}

pub fn closest_location(alm: Almanac) -> i64 {
    let locs = locations(&alm);
    println!("Ending locations: {:?}", locs);
    *locs.iter().min().expect("Empty Locations")
}

fn convert_by(i: i64, conversions: &Vec<Mapping>) -> i64 {
    let mut stuff = conversions.iter().filter(|&c| c.source_range.contains(&i));
    if let Some(m) = stuff.next() {
        let diff = m.destination_range.start - m.source_range.start;
        i + diff
    } else {
        i
    }
}

fn locations(alm: &Almanac) -> Vec<i64> {
    alm.seeds
        .iter()
        .map(|&seed| convert_by(seed, &alm.seed_to_soil))
        .map(|soil| convert_by(soil, &alm.soil_to_fertilizer))
        .map(|fertilizer| convert_by(fertilizer, &alm.fertilizer_to_water))
        .map(|water| convert_by(water, &alm.water_to_light))
        .map(|light| convert_by(light, &alm.light_to_temperature))
        .map(|temperature| convert_by(temperature, &alm.temperature_to_humidity))
        .map(|humidity| convert_by(humidity, &alm.humidity_to_location))
        .collect::<Vec<i64>>()
}
