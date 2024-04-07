mod data;
mod model;
mod parsing;

pub fn run() {
    let data = data::from_file();
    let records = parsing::parse(data);
    let mut total = 0;
    for (index, rec) in records.iter().enumerate() {
        println!(
            "{index}) {:?} => {:?}",
            rec.parsed_conditions, rec.damaged_runs
        );
        let cfgs = rec.configurations();
        total += cfgs.len();
        println!("Found valid {} configurations", cfgs.len());
        for c in cfgs {
            println!("\t{:?}", c);
        }
        println!();
    }
    println!("Sum of configurations: {}", total)
}
