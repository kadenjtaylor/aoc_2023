#[derive(Debug)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
    pub content: char,
}

#[derive(Debug)]
pub struct CellGroup {
    pub cells: Vec<Cell>,
    pub num: String,
}

fn retrieve_cells() -> Vec<Cell> {
    let lines = include_str!("../../resources/problem_3.txt").lines();
    let mut cells: Vec<Cell> = vec![];
    for (num, line) in lines.enumerate() {
        line.chars().enumerate().for_each(|(char_num, c)| {
            cells.push(Cell {
                y: num as i32,
                x: char_num as i32,
                content: c,
            })
        })
    }
    cells
}

fn partition_cell_types(cells: Vec<Cell>) -> (Vec<Cell>, Vec<Cell>, Vec<Cell>) {
    let mut numbers: Vec<Cell> = vec![];
    let mut symbols: Vec<Cell> = vec![];
    let mut empties: Vec<Cell> = vec![];
    for c in cells {
        if c.content.is_ascii_digit() {
            numbers.push(c);
        } else if c.content == '.' {
            empties.push(c);
        } else {
            symbols.push(c);
        }
    }
    (numbers, symbols, empties)
}

struct Acc {
    nums: Vec<CellGroup>,
    partial: Vec<Cell>,
}

fn flush(acc: Acc, new_partial: Vec<Cell>) -> Acc {
    let mut new_cells = vec![];
    new_cells.extend(acc.partial);
    let num_str: String = new_cells
        .iter()
        .map(|cell: &Cell| cell.content.to_string())
        .collect::<Vec<String>>()
        .join("");
    let mut new_nums = vec![];
    new_nums.extend(acc.nums);
    new_nums.extend(vec![CellGroup {
        cells: new_cells,
        num: num_str,
    }]);
    Acc {
        nums: new_nums,
        partial: new_partial,
    }
}

fn combine_numbers(mut number_cells: Vec<Cell>) -> Vec<CellGroup> {
    number_cells.sort_by(|a, b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)));
    let mut acc = Acc {
        nums: vec![],
        partial: vec![],
    };
    for next in number_cells {
        match acc.partial.last() {
            None => {
                acc.partial.push(next);
            }
            Some(cell) => {
                if next.x - cell.x == 1 {
                    acc.partial.push(next);
                } else {
                    acc = flush(acc, vec![next]);
                }
            }
        }
    }
    acc = flush(acc, vec![]);
    acc.nums
}

pub struct Schematic {
    pub number_groups: Vec<CellGroup>,
    pub symbols: Vec<Cell>,
    pub empties: Vec<Cell>,
}

pub fn produce_schematic() -> Schematic {
    // Parse the given file into cells
    let cells = retrieve_cells();

    // Decide which ones are numbers and which are symbols
    let (numbers, symbols, empties) = partition_cell_types(cells);

    // The schematic is just the above with the numbers all put together
    Schematic {
        number_groups: combine_numbers(numbers),
        symbols,
        empties,
    }
}

pub fn does_border(g: &CellGroup, cell: &Cell) -> bool {
    g.cells.iter().any(|gc| {
        let x_dist = gc.x.abs_diff(cell.x);
        let y_dist = gc.y.abs_diff(cell.y);
        x_dist <= 1 && y_dist <= 1
    })
}

pub fn is_part(g: &CellGroup, symbols: &Vec<Cell>) -> bool {
    symbols.iter().any(|c| does_border(g, c))
}
