mod day01;
mod day02;
mod day05;
mod day09;

criterion::criterion_main!(
    day01::benches,
    day02::benches,
    day05::benches,
    day09::benches
);
