use crate::aoc::{conv, io, Int2, Solution};

pub fn solve() -> Solution {
    let pairs = data(true);

    let mut count1 = 0;
    let mut count2 = 0;
    for pair in pairs {
        // Part 1
        if is_superset_pair(pair) {
            count1 += 1;
        }

        // Part 2 
        if is_overlapping_pair(pair) {
            count2 += 1;
        }
    }

    io::solution(count1, count2)
}

type Pair = (Int2, Int2);

fn data(full: bool) -> Vec<Pair> {
    io::read_lines(22, 4, full)
        .into_iter()
        .map(|line| {
            let p: Vec<&str> = line.split(",").collect();
            let p1 = conv::to_int2(p[0], "-");
            let p2 = conv::to_int2(p[1], "-");
            (p1, p2)
        })
        .collect()
}

fn is_superset_pair(p: Pair) -> bool {
    let (r1, r2) = p;
    return is_superset_range(r1, r2) || is_superset_range(r2, r1);
}

fn is_superset_range(r1: Int2, r2: Int2) -> bool {
    let (s1, e1) = r1;
    let (s2, e2) = r2;
    s1 <= s2 && e2 <= e1
}

fn is_overlapping_pair(p: Pair) -> bool {
    let ((s1, e1), (s2, e2)) = p;
    if s1 < s2 {
        s2 <= e1
    } else {
        s1 <= e2
    }
}