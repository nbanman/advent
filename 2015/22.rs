use advent::prelude::*;
use crate::Spell::*;

const PLAYER_HP: isize = 50;
const STARTING_MANA: isize = 500;

#[derive(Clone, Copy)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    fn mana_cost(&self) -> isize {
        match self {
            MagicMissile => 53,
            Drain => 73,
            Shield => 113,
            Poison => 173,
            Recharge => 229,
        }
    }
    fn duration(&self) -> isize {
        match self {
            MagicMissile => 1,
            Drain => 1,
            Shield => 6,
            Poison => 6,
            Recharge => 5,
        }
    }
    fn effect(&self) -> isize {
        match self {
            MagicMissile => 4,
            Drain => 2,
            Shield => 7,
            Poison => 3,
            Recharge => 101,
        }
    }

    fn entries() -> &'static [Spell] {
        &[
            MagicMissile,
            Drain,
            Shield,
            Poison,
            Recharge,
        ]
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct State {
    mana_spent: isize,
    player_hp: isize,
    boss_hp: isize,
    current_mana: isize,
    shield: isize,
    poison: isize,
    recharge: isize,
}
impl State {
    fn available_mana(&self) -> isize {
        self.current_mana + (if self.recharge > 0 { Recharge.effect() } else { 0 })
    }
}

fn player_turn(state: &State, spell: Spell, constant_drain: isize) -> State {
    if state.player_hp.checked_sub(constant_drain) == Some(0) {
        return State { player_hp: 0, ..*state }
    }
    let new_player_hp = state.player_hp +
        (if let Drain = spell { spell.effect() } else { 0 }) -
        constant_drain;
    let new_boss_hp = state.boss_hp -
        if state.poison > 0 { Poison.effect() } else { 0 } -
        if let MagicMissile = spell { spell.effect() } else { 0 } -
        if let Drain = spell { spell.effect() } else { 0 };
    let new_current_mana = state.current_mana -
        spell.mana_cost() +
        if state.recharge > 0 { Recharge.effect() } else { 0 };
    let new_mana_spent = state.mana_spent + spell.mana_cost();
    let new_shield = if let Shield = spell { spell.duration() } else { state.shield - 1 };
    let new_poison = if let Poison = spell { spell.duration() } else { state.poison - 1 };
    let new_recharge = if let Recharge = spell { spell.duration() } else { state.recharge - 1 };
    State {
        mana_spent: new_mana_spent,
        player_hp: new_player_hp,
        boss_hp: new_boss_hp,
        current_mana: new_current_mana,
        shield: new_shield,
        poison: new_poison,
        recharge: new_recharge,
    }
}

fn boss_turn(state: &State, damage: isize) -> State {
    if state.player_hp <= 0 { return (*state).clone(); }
    let armor = if state.shield > 0 { Shield.effect() } else { 0 };
    let new_player_hp = state.player_hp - (damage - armor).max(1);
    let new_boss_hp = state.boss_hp - if state.poison > 0 { Poison.effect() } else { 0 };
    let new_current_mana = state.current_mana +
        if state.recharge > 0 { Recharge.effect() } else { 0 };
    State {
        mana_spent: state.mana_spent,
        player_hp: new_player_hp,
        boss_hp: new_boss_hp,
        current_mana: new_current_mana,
        shield: state.shield - 1,
        poison: state.poison - 1,
        recharge: state.recharge - 1,
    }
}
fn parse_input(input: &str) -> (isize, isize) {
    input.get_numbers().tuples().next().unwrap()
}

fn default_input() -> (isize, isize) {
    parse_input(include_input!(2015 / 22))
}

fn solve(boss_hp: isize, damage: isize, constant_drain: isize) -> isize {
    let mut states = BinaryHeap::new();
    states.push(Reverse(State {
        mana_spent: 0,
        player_hp: PLAYER_HP,
        boss_hp,
        current_mana: STARTING_MANA,
        shield: 0,
        poison: 0,
        recharge: 0,
    }));
    let mut lowest_mana_win = isize::MAX;
    while let Some(Reverse(current)) = states.pop() {
        let next_states = Spell::entries().iter()
            .filter(|spell| {
                if current.available_mana() < spell.mana_cost() {
                    false
                } else {
                    match spell {
                        Shield => current.shield <= 1,
                        Poison => current.poison <= 1,
                        Recharge => current.recharge <= 1,
                        _ => true
                    }
                }
            })
            .map(|spell| {
                let player_turn = player_turn(&current, *spell, constant_drain);
                boss_turn(&player_turn, damage)
            })
            .filter(|turn| {
                if turn.boss_hp <= 0 {
                    lowest_mana_win = min(lowest_mana_win, turn.mana_spent);
                    false
                } else if turn.player_hp <= 0 ||
                    turn.available_mana() < 53 ||
                    turn.mana_spent > lowest_mana_win {
                    false
                } else { true }
            });
        for next_state in next_states {
            states.push(Reverse(next_state))
        }
    }
    lowest_mana_win
}

fn part1((boss_hp, damage): (isize, isize)) -> isize {
    solve(boss_hp, damage, 0)
}

fn part2((boss_hp, damage): (isize, isize)) -> isize {
    solve(boss_hp, damage, 1)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1824);
    assert_eq!(part2(input), 1937);
}
