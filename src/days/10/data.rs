#[allow(dead_code)]
pub fn get_easy_data() -> &'static str {
    ".....
.S-7.
.|.|.
.L-J.
....."
}

#[allow(dead_code)]
pub fn get_complex_data() -> &'static str {
    "..F7.
  .FJ|.
  SJ.L7
  |F--J
  LJ..."
}

#[allow(dead_code)]
pub fn get_file_data() -> &'static str {
    include_str!("../../../resources/pipe_grid.txt")
}

#[allow(dead_code)]
pub fn get_part2_example() -> &'static str {
    "...........
    .S-------7.
    .|F-----7|.
    .||.....||.
    .||.....||.
    .|L-7.F-J|.
    .|..|.|..|.
    .L--J.L--J.
    ..........."
}

#[allow(dead_code)]
pub fn get_part2_example2() -> &'static str {
    "FF7FSF7F7F7F7F7F---7
    L|LJ||||||||||||F--J
    FL-7LJLJ||||||LJL-77
    F--JF--7||LJLJ7F7FJ-
    L---JF-JLJ.||-FJLJJ7
    |F|F-JF---7F7-L7L|7|
    |FFJF7L7F-JF7|JL---7
    7-L-JL7||F7|L7F-7F7|
    L.L7LFJ|||||FJL7||LJ
    L7JLJL-JLJLJL--JLJ.L"
}