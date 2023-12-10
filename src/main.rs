use std::time::Instant;

fn main() {
    let inst = Instant::now();
    let inp = include_str!("../day6data.txt");

    let e: Vec<Vec<u32>> = inp
        .split_terminator(&['\n', '\r'])
        .filter(|e| !e.is_empty())
        .map(|e| {
            e.split(' ')
                .filter(|e| !e.is_empty())
                .filter_map(|e| e.parse::<u32>().ok())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<_>>();

    let times = &e[0];
    let distances = &e[1];

    let mut result = 1;
    for (time, distance) in times.iter().zip(distances) {
        let mut chances = 0;
        for i in 0..*time {
            if (time - i) * i > *distance {
                chances += 1;
            }
        }
        result *= chances;
    }

    println!("{result}");
    println!("Time elapsed: {:?}", inst.elapsed())
}
