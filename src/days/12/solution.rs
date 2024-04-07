mod data;
mod parsing;

pub fn run() {
    let data = data::from_file();
    let records = parsing::parse(data);
    let total = records
        .iter()
        .map(|r| r.configurations().len())
        .sum::<usize>();
    println!("Sum of configurations: {}", total)
}
