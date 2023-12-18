#[derive(Debug)]
struct Cell {
    x: i32,
    y: i32,
    content: char,
}

#[derive(Debug)]
struct CellGroup<'a> {
    cells: Vec<&'a Cell>,
    num: String,
}

fn partition_cell_types(cells: &Vec<Cell>) -> (Vec<&Cell>, Vec<&Cell>, Vec<&Cell>) {
    let mut numbers = vec![];
    let mut symbols = vec![];
    let mut empties = vec![];
    cells.iter().for_each(|c| {
        if c.content.is_ascii_digit() {
            numbers.push(c);
        } else if c.content == '.' {
            empties.push(c);
        } else {
            symbols.push(c);
        }
    });
    (numbers, symbols, empties)
}

struct Acc<'a> {
    nums: Vec<CellGroup<'a>>,
    partial: Vec<&'a Cell>,
}

// TODO: Refactor - this is ugly as hell
fn combine_numbers(number_cells: Vec<&Cell>) -> Vec<CellGroup> {
    // println!("Figuring out which cells belong to the same group...");
    // println!("They'll be equal in y, and 1 apart in x");
    let mut my_cells = number_cells.clone();
    my_cells.sort_by(|&a, &b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)));
    let mut acc = Acc {
        nums: vec![],
        partial: vec![],
    };
    my_cells.iter().for_each(|next| {
        // println!("Accumulating: {:?}", next);
        if acc.partial.is_empty() {
            acc.partial.extend(vec![next]);
            // println!("Added {:?} to partial", next);
        } else if let Some(cell) = acc.partial.last() {
            // println!("{:?} already in partial... now what?", cell);
            if next.x - cell.x == 1 {
                // println!("This is a neighbor! Adding it next to the last one!");
                acc.partial.extend(vec![next]);
            } else {
                // println!("Not a neighbor! Flushing partial and beginning a new one!");
                let mut new_cells = vec![];
                new_cells.extend(acc.partial.iter());
                acc.partial = vec![next];
                let num_str = new_cells
                    .iter()
                    .map(|cell: &&Cell| cell.content.to_string())
                    .collect::<Vec<String>>()
                    .join("");
                let new_cellgroup = vec![CellGroup {
                    cells: new_cells,
                    num: num_str,
                }];
                // println!("Created: {:?}", new_cellgroup);
                acc.nums.extend(new_cellgroup);
            }
        }
    });
    // TODO: This is just the 'flush partial' routine again...
    let mut new_cells = vec![];
    new_cells.extend(acc.partial.iter());
    acc.partial = vec![];
    let num_str = new_cells
        .iter()
        .map(|cell: &&Cell| cell.content.to_string())
        .collect::<Vec<String>>()
        .join("");
    let new_cellgroup = vec![CellGroup {
        cells: new_cells,
        num: num_str, // todo: fail
    }];
    // println!("Created: {:?}", new_cellgroup);
    acc.nums.extend(new_cellgroup);
    acc.nums
}

fn does_border(g: &CellGroup, cell: &Cell) -> bool {
    g.cells.iter().any(|gc| {
        let x_dist = gc.x.abs_diff(cell.x);
        let y_dist = gc.y.abs_diff(cell.y);
        x_dist <= 1 && y_dist <= 1
    })
}

fn is_part(g: &CellGroup, symbols: &Vec<&Cell>) -> bool {
    symbols.iter().any(|c| does_border(g, c))
}

pub fn run() {
    let lines = include_str!("../resources/problem_3.txt").lines();

    // Parse into cells
    let mut cells: Vec<Cell> = vec![];
    lines.enumerate().for_each(|(lin_num, l)| {
        l.chars().enumerate().for_each(|(char_num, c)| {
            cells.push(Cell {
                y: lin_num as i32,
                x: char_num as i32,
                content: c,
            })
        })
    });

    // Decide which ones are numbers and which are symbols
    let (number_cells, symbol_cells, empty_cells) = partition_cell_types(&cells);

    println!(
        "Number Cells: {}, Symbol Cells: {}, Empty Cells: {}",
        number_cells.len(),
        symbol_cells.len(),
        empty_cells.len()
    );

    let numbers = combine_numbers(number_cells);

    println!("Numbers found: {}", numbers.len());

    let part_numbers = numbers
        .iter()
        .filter(|&n| is_part(n, &symbol_cells))
        .collect::<Vec<&CellGroup>>();

    println!("Part Numbers found: {}", part_numbers.len());
    let sum = part_numbers
        .iter()
        .map(|cg| cg.num.parse::<i32>().expect("FAILED TO PARSE NUMBER"))
        .sum::<i32>();
    println!("Sum of part numbers: {}", sum);
}
