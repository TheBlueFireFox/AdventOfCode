#![allow(dead_code)]

use std::collections::VecDeque;

use strum::{EnumCount, EnumIter, FromRepr, IntoEnumIterator};

const INPUT: &str = include_str!("../input.txt");
const ME: Hero = Hero {
    hit_points: 50,
    mana: 500,
    armor: 0,
    used_mana: 0,
};

fn main() {
    println!("result <{}>", two());
}

fn one() -> Mana {
    let boss = input().unwrap();
    battle(ME, boss, false)
}

fn two() -> Mana {
    let boss = input().unwrap();
    battle(ME, boss, true)
}

fn battle(hero: Hero, boss: Boss, hard: bool) -> Mana {
    let mut min_mana = Mana::MAX;
    let mut queue = VecDeque::with_capacity(Attacks::COUNT * 4);

    for attack in Attacks::iter() {
        queue.push_back(State {
            hero: hero.clone(),
            boss: boss.clone(),
            attack,
            active: ActiveEffects::default(),
        });
    }

    while let Some(mut state) = queue.pop_back() {
        if hard {
            state.hero.hit_points = state.hero.hit_points.saturating_sub(1);
            if state.hero.hit_points == 0 {
                continue;
            }
        }

        //// HERO's turn
        if let Some(mana_used) = pre_effects(&mut state.hero, &mut state.boss, &mut state.active) {
            min_mana = std::cmp::min(min_mana, mana_used);
            continue;
        }

        // check if the hero's attack has killed the boss
        if hero_attack(&mut state) {
            min_mana = std::cmp::min(min_mana, state.hero.used_mana);
            continue;
        }

        //// BOSS's turn
        if let Some(mana_used) = pre_effects(&mut state.hero, &mut state.boss, &mut state.active) {
            min_mana = std::cmp::min(min_mana, mana_used);
            continue;
        }

        // check if boss's attack has killed the hero
        if boss_attack(&mut state.hero, &state.boss) {
            continue;
        }

        // hero prepares for next attack
        for attack in Attacks::iter() {
            // check that the hero has the mana needed to use the spell
            if state.hero.mana <= attack.mana() {
                continue;
            }
            // check that the next attack will not be more expensive
            // then the current best
            if state.hero.used_mana + attack.mana() >= min_mana {
                continue;
            }

            queue.push_back(State {
                hero: state.hero.clone(),
                boss: state.boss.clone(),
                attack,
                active: state.active.clone(),
            });
        }
    }
    min_mana
}

fn pre_effects(hero: &mut Hero, boss: &mut Boss, active: &mut ActiveEffects) -> Option<Mana> {
    // do effects on boss
    for (idx, turns) in active
        .iter_mut()
        .enumerate()
        .filter(|(_, turns)| **turns > 0)
    {
        *turns -= 1;

        match Attacks::from_repr(idx).unwrap() {
            Attacks::MagicMissile | Attacks::Drain => {
                unreachable!("this effect should be done instantly")
            }
            Attacks::Shield => {
                if *turns == 0 {
                    // effect is done
                    hero.armor -= 7;
                }
            }
            Attacks::Poison => {
                boss.hit_points = boss.hit_points.saturating_sub(3);
                if boss.hit_points == 0 {
                    // we have a winning path
                    return Some(hero.used_mana);
                }
            }
            Attacks::Recharge => {
                hero.mana += 101;
            }
        }
    }
    None
}

fn boss_attack(hero: &mut Hero, boss: &Boss) -> bool {
    // boss
    hero.hit_points = hero
        .hit_points
        .saturating_sub(std::cmp::max(boss.damage.saturating_sub(hero.armor), 1));

    // is this a loosing path?
    hero.hit_points == 0
}

fn hero_attack(state: &mut State) -> bool {
    let (id, att, cost, durr) = (
        state.attack as usize,
        state.attack,
        state.attack.mana(),
        state.attack.duration(),
    );

    // set the duration of the effect
    state.active[id] = durr;

    // make sure that the hero has the mana needed for this
    // it should already be checked, but checking it again
    // should not be an issue
    assert!(
        state.hero.mana >= cost,
        "hero's mana has run out during his attack"
    );

    state.hero.mana -= cost;
    state.hero.used_mana += cost;

    let mut do_damage = |damage| {
        // do the damage
        state.boss.hit_points = state.boss.hit_points.saturating_sub(damage);
        state.boss.hit_points == 0
    };

    match att {
        Attacks::MagicMissile => do_damage(4),
        Attacks::Drain => {
            state.hero.hit_points += 2;
            do_damage(2)
        }
        Attacks::Shield => {
            state.hero.armor += 7;
            false
        }
        Attacks::Poison | Attacks::Recharge => {
            // no direct effect
            false
        }
    }
}

type Mana = u16;
type Timer = u8;
type ActiveEffects = [Timer; Attacks::COUNT];

#[derive(Debug, EnumCount, EnumIter, FromRepr, Clone, Copy)]
#[repr(usize)]
enum Attacks {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Attacks {
    fn mana(&self) -> Mana {
        match self {
            Attacks::MagicMissile => 53,
            Attacks::Drain => 73,
            Attacks::Shield => 113,
            Attacks::Poison => 173,
            Attacks::Recharge => 229,
        }
    }

    fn duration(&self) -> Timer {
        match self {
            Attacks::MagicMissile => 0,
            Attacks::Drain => 0,
            Attacks::Shield => 6,
            Attacks::Poison => 6,
            Attacks::Recharge => 5,
        }
    }
}

#[derive(Debug, Clone)]
pub struct State {
    hero: Hero,
    boss: Boss,
    active: ActiveEffects,
    attack: Attacks,
}

#[derive(Debug, Clone)]
pub struct Hero {
    pub hit_points: u8,
    pub armor: u8,
    pub mana: Mana,
    pub used_mana: Mana,
}

#[derive(Debug, Clone)]
pub struct Boss {
    pub hit_points: u8,
    pub damage: u8,
}

pub fn input() -> Option<Boss> {
    let mut it = INPUT.lines().filter_map(|s| {
        let (_, v) = s.rsplit_once(": ")?;
        v.parse().ok()
    });

    Some(Boss {
        hit_points: it.next()?,
        damage: it.next()?,
    })
}
