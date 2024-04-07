#[derive(Debug)]
pub struct Row {
    pub parsed_conditions: Vec<char>,
    pub damaged_runs: Vec<usize>,
}

fn stack_on(acc: Vec<usize>, current_run: usize) -> Vec<usize> {
    let mut new_acc = vec![];
    new_acc.extend(acc);
    new_acc.extend([current_run]);
    new_acc
}

fn get_counts(row: &Vec<char>) -> Option<Vec<usize>> {
    let init = Some((vec![], 0));
    let result: Option<(Vec<usize>, usize)> =
        row.iter().fold(init, |acc, item| match (acc, item) {
            (None, _) => None,
            (_, '?') => None,
            (Some((acc, 0)), '.') => Some((acc, 0)),
            (Some((acc, current_run)), '.') => Some((stack_on(acc, current_run), 0)),
            (Some((acc, current_run)), '#') => Some((acc, current_run + 1)),
            _ => todo!("Not sure how I messed this up"),
        });
    let counts = result.map(|(acc, current_run)| match (acc, current_run) {
        (acc, 0) => acc,
        (acc, current_run) => stack_on(acc, current_run),
    });
    // println!("{:?} => {:?}", row, counts);
    counts
}

impl Row {
    fn generate_options(self: &Self) -> Vec<Vec<char>> {
        self.parsed_conditions
            .iter()
            .fold(vec![], |acc, next_cond| {
                match next_cond {
                    &cond @ ('.' | '#') => {
                        if acc.len() == 0 {
                            vec![vec![cond]]
                        } else {
                            acc.into_iter()
                                .map(|v| [v, vec![cond.clone()]].concat())
                                .collect()
                        }
                    }
                    _ => {
                        if acc.len() == 0 {
                            vec![vec!['.'], vec!['#']]
                        } else {
                            let good_half: Vec<Vec<char>> = acc
                                .clone()
                                .into_iter()
                                .map(|v| [v.as_slice(), ['.'].as_slice()].concat())
                                .collect();
                            // println!("Good Half: {:?}", good_half);
                            let bad_half: Vec<Vec<char>> = acc
                                .into_iter()
                                .map(|v| [v.as_slice(), ['#'].as_slice()].concat())
                                .collect();
                            // println!("Bad Half: {:?}", bad_half);
                            vec![good_half.as_slice(), bad_half.as_slice()].concat()
                        }
                    }
                }
            })
    }

    fn is_valid(self: &Self) -> bool {
        let derived_runs = get_counts(&self.parsed_conditions).expect("Failed to get counts");
        derived_runs == self.damaged_runs
    }

    pub fn configurations(self: &Self) -> Vec<Vec<char>> {
        let options = self.generate_options();
        // println!("Generated options: {}", options.len());
        let candidate_rows: Vec<Row> = options
            .into_iter()
            .map(|conditions| Row {
                parsed_conditions: conditions,
                damaged_runs: self.damaged_runs.clone(),
            })
            .collect();
        // println!("Generated {} candidate rows", candidate_rows.len());
        let valid_possibilities = candidate_rows.iter().filter(|row| row.is_valid());
        // println!("Valid Possibilities: {}", valid_possibilities);
        valid_possibilities
            .map(|row| {
                let thing: Vec<char> = row
                    .parsed_conditions
                    .iter()
                    .map(|opt_c| opt_c.clone())
                    .collect();
                thing
            })
            .collect()
    }
}

pub type DamageRecords = Vec<Row>;
