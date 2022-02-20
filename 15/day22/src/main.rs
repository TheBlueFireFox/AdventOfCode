#![allow(dead_code)]

use strum::{EnumCount, EnumIter, FromRepr, IntoEnumIterator};

const INPUT: &str = include_str!("../input.txt");
const ME: Hero = Hero {
    hit_points: 50,
    mana: 500,
    armor: 0,
    used_mana: 0,
};

fn main() {
    println!("result <{}>", one());
}

fn one() -> Mana {
    let boss = input().unwrap();
    one_inner(ME, boss)
}

fn one_inner(hero: Hero, boss: Boss) -> Mana {
    minimax(hero, boss, 0, ActiveEffects::default()).unwrap()
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

fn minimax(mut hero: Hero, mut boss: Boss, depth: u16, mut active: ActiveEffects) -> Option<Mana> {
    // run preround effects / and check for a win
    if let Some(mana) = pre_effects(&mut hero, &mut boss, &mut active) {
        return Some(mana);
    }

    if depth % 2 == 0 {
        hero_attack(hero, boss, depth + 1, active)
    } else {
        boss_attack(hero, boss, depth + 1, active)
    }
}

fn boss_attack(mut hero: Hero, boss: Boss, depth: u16, active: ActiveEffects) -> Option<Mana> {
    // boss
    hero.hit_points = hero
        .hit_points
        .saturating_sub(std::cmp::max(boss.damage.saturating_sub(hero.armor), 1));

    // this is a loosing path
    if hero.hit_points == 0 {
        None
    } else {
        // simple recursion no branches needed for boss move
        minimax(hero, boss, depth, active)
    }
}

fn hero_attack(hero: Hero, boss: Boss, depth: u16, active: ActiveEffects) -> Option<Mana> {
    // mine
    let mut prevs = None;

    for (id, att, cost, durr) in Attacks::iter()
        .map(|att| (att as usize, att, att.mana(), att.duration()))
        // check if effect is active => don't try to use it again
        // until after the cool down
        .filter(|(id, _, _, _)| active[*id] == 0)
    {
        let mut hero = hero.clone();
        let mut boss = boss.clone();
        let mut active = active.clone();

        // set the duration of the effect
        active[id] = durr;

        // check if there is enough mana for effect
        if cost > hero.mana {
            // no path
            continue;
        }

        hero.mana -= cost;
        hero.used_mana += cost;

        // check if new path will be cheaper then previous winning one
        // (pruning step)
        if let Some(other_mana) = prevs {
            if other_mana > hero.used_mana {
                continue;
            }
        }

        let mut check_win = |mana| {
            // found a winning path
            // check if previous win exists and if
            // it was better or worse
            prevs = Some(prevs.map_or(mana, |other| std::cmp::min(other, mana)));
            eprintln!("found winning path <{:?}>", mana);
        };

        let mut do_damage = |damage| {
            // do the damage
            boss.hit_points = boss.hit_points.saturating_sub(damage);
            if boss.hit_points == 0 {
                // winning path, so check if better score
                check_win(hero.used_mana);
                true
            } else {
                false
            }
        };

        match att {
            Attacks::MagicMissile => {
                if do_damage(4) {
                    continue;
                }
            }
            Attacks::Drain => {
                if do_damage(2) {
                    continue;
                }
                hero.hit_points += 2;
            }
            Attacks::Shield => {
                hero.armor += 7;
            }
            Attacks::Poison | Attacks::Recharge => {
                // no direct effect
            }
        }

        // try to follow this path to the end
        if let Some(res) = minimax(hero, boss, depth, active) {
            check_win(res);
        }
    }
    prevs
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
pub struct Hero {
    pub hit_points: u8,
    pub armor: u8,
    pub mana: u16,
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
