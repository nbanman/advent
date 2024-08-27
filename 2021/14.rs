use advent::prelude::*;

type PropagationRules = HashMap<(u8, u8), ((u8, u8), (u8, u8))>;
type ProteinPairs = HashMap<(u8, u8), usize>;

fn parse_input(input: &str) -> (PropagationRules, ProteinPairs, (u8, u8)) {
    let (template, rules) = input.split_once("\n\n").unwrap();
    let template = template.as_bytes();
    let edge_proteins = (template[0], template[template.len() - 1]);


    let mut protein_pairs = ProteinPairs::new();
    for pairs in template.windows(2) {
        if let &[a, b] = pairs {
            *protein_pairs.entry((a, b)).or_insert(0) += 1;
        }
    }

    let rules = rules.as_bytes().into_iter()
        .filter(|c| c.is_ascii_alphabetic())
        .array_chunked()
        .map(|[&a, &b, &c]| ((a, b), ((a, c), (c, b))))
        .collect();

    (rules, protein_pairs, edge_proteins)
}

fn default_input() -> (PropagationRules, ProteinPairs, (u8, u8)) {
    parse_input(include_input!(2021 / 14))
}

fn solve(
    (progagation_rules, protein_pairs, edge_proteins): (PropagationRules, ProteinPairs, (u8, u8)),
    steps: usize,
) -> usize {
    let polymerized = polymerize(protein_pairs, steps, &progagation_rules);
    let count = count_proteins(polymerized, edge_proteins);
    let (min, max) = min_max(&count);
    max - min
}

fn polymerize(
    protein_pairs: ProteinPairs, steps: usize, propagation_rules: &PropagationRules,
) -> ProteinPairs {
    if steps == 0 {
        protein_pairs
    } else {
        let mut new_pairs = ProteinPairs::new();
        for (protein, amt) in protein_pairs {
            let &(a, b) = propagation_rules.get(&protein).unwrap();
            *new_pairs.entry(a).or_insert(0) += amt;
            *new_pairs.entry(b).or_insert(0) += amt;
        }
        polymerize(new_pairs, steps - 1, propagation_rules)
    }
}

fn count_proteins(protein_pairs: ProteinPairs, edge_proteins: (u8, u8)) -> Vec<usize> {
    let mut double_counts = HashMap::new();
    for ((a, b), amt) in protein_pairs {
        *double_counts.entry(a).or_insert(0usize) += amt;
        *double_counts.entry(b).or_insert(0) += amt;
    }
    *double_counts.entry(edge_proteins.0).or_insert(0) += 1;
    *double_counts.entry(edge_proteins.1).or_insert(0) += 1;

    double_counts.values()
        .map(|it| it / 2)
        .collect()
}

fn part1(input: (PropagationRules, ProteinPairs, (u8, u8))) -> usize {
    solve(input, 10)
}

fn part2(input: (PropagationRules, ProteinPairs, (u8, u8))) -> usize {
    solve(input, 40)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C",
    );
    assert_eq!(part1(input.clone()), 1588);
    assert_eq!(part2(input), 2188189693529);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 3555);
    assert_eq!(part2(input), 4439442043739);
}
