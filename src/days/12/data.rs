#[allow(dead_code)]
pub fn clean_example() -> &'static str {
    "#.#.### 1,1,3
.#...#....###. 1,1,3
.#.###.#.###### 1,3,1,6
####.#...#... 4,1,1
#....######..#####. 1,6,5
.###.##....# 3,2,1"
}

#[allow(dead_code)]
pub fn damaged_example() -> &'static str {
    "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"
}

#[allow(dead_code)]
pub fn from_file() -> &'static str {
    include_str!("../../../resources/damaged_parts.txt")
}
