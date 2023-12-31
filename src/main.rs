use std::time::Instant;

fn main() {
    let inst = Instant::now();
    let inp = include_str!("../day6data.txt");

    let e: Vec<Vec<u16>> = inp
        .split_terminator(&['\n', '\r'])
        .filter(|e| !e.is_empty())
        .map(|e| {
            e.split(' ')
                .filter(|e| !e.is_empty())
                .filter_map(|e| e.parse::<u16>().ok())
                .collect::<Vec<u16>>()
        })
        .collect::<Vec<_>>();

    let (times, distances) = e.split_at(1);

    let result = times[0]
        .iter()
        .zip(distances[0].iter())
        .map(|(time, distance)| (0..*time).filter(|i| (time - *i) * *i > *distance).count())
        .product::<usize>();

    let end = inst.elapsed();

    println!("{}", result);
    println!("Time elapsed: {:?}", end);
}
