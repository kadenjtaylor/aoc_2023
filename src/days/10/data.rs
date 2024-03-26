pub fn get_easy_data() -> &'static str {
    ".....
.S-7.
.|.|.
.L-J.
....."
}

pub fn get_complex_data() -> &'static str {
    "..F7.
  .FJ|.
  SJ.L7
  |F--J
  LJ..."
}

pub fn get_file_data() -> &'static str {
    include_str!("../../../resources/pipe_grid.txt")
}
