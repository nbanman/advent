use advent::prelude::*;

#[derive(Clone, Debug)]
struct Claim {
    tl: Vector2,
    br: Vector2,
}
fn parse_input(input: &str) -> (Vec<Claim>, Vec<i64>, i64) {
    let claims: Vec<_> = input
        .get_numbers()
        .array_chunked()
        .map(|[_, x, y, w, h]| {
            Claim { tl: vector![x, y], br: vector![x + w - 1, y + h - 1] }
        })
        .collect();

    let width = claims.iter().map(|claim| claim.br.x).max().unwrap() + 1;
    let height = claims.iter().map(|claim| claim.br.y).max().unwrap() + 1;

    let mut skein = vec![0i64; (width * height) as usize];
    for claim in claims.iter() {
        for y in claim.tl.y..=claim.br.y {
            for x in claim.tl.x..=claim.br.x {
                let idx = (y * width + x) as usize;
                skein[idx] += 1;
            }
        }
    }
    (claims, skein, width)
}

fn default_input() -> (Vec<Claim>, Vec<i64>, i64) {
    parse_input(include_input!(2018 / 03))
}

fn part1((_, skein, _): (Vec<Claim>, Vec<i64>, i64)) -> usize {
    skein.into_iter().filter(|it| it > &1).count()
}

fn part2((claims, skein, width): (Vec<Claim>, Vec<i64>, i64)) -> usize {
    1 + claims.into_iter().enumerate()
        .find(|(_, claim)| {
            (claim.tl.y..=claim.br.y).cartesian_product(claim.tl.x..=claim.br.x)
                .all(|(y, x)| {
                    let pos = y * width + x;
                    skein[pos as usize] == 1
                })
        })
        .unwrap()
        .0
}


fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}
#[test]
fn example() {
    let input = parse_input(
        "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2",
    );
    assert_eq!(part1(input.clone()), 4);
    assert_eq!(part2(input), 3);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 110891);
    assert_eq!(part2(input), 297);
}
