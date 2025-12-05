pub fn merge_ranges<T: Ord+Copy>(mut ranges: Vec<(T, T)>) -> Vec<(T, T)> {
    let mut result: Vec<(T, T)> = Vec::new();
    ranges.sort();
    let mut curr_start = ranges[0].0;
    let mut curr_end = ranges[0].1;
    for i in 1..ranges.len() {
        let (next_start, next_end) = ranges[i];
        if curr_start <= next_start && next_end <= curr_end {
            continue; // subset
        }
        if next_start <= curr_end {
            curr_end = next_end; // overlap
        } else {
            result.push((curr_start, curr_end));
            (curr_start, curr_end) = (next_start, next_end);
        }
    }
    result.push((curr_start, curr_end));
    result
}