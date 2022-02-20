#![allow(dead_code)]

use strum::{EnumCount, EnumIter, FromRepr, IntoEnumIterator};

const INPUT: &str = include_str!("../input.txt");
const ME: Person = Person {
    hit_points: 50,
    mana: 500,
    damage: 0,
    armor: 0,
};

fn main() {
    println!("result <{}>", one());
}

fn one() -> usize {
    let boss = input().unwrap();
    minimax(ME, boss, 0, 0, [0; Attacks::COUNT]).unwrap()
}

fn pre_effects(
    me: &mut Person,
    boss: &mut Person,
    mana: &mut usize,
    active: &mut [usize; Attacks::COUNT],
) -> Option<usize> {
    // do effects on boss
    for (idx, count) in active
        .iter_mut()
        .enumerate()
        .filter(|(_, count)| **count > 0)
    {
        *count -= 1;

        match Attacks::from_repr(idx).unwrap() {
            Attacks::MagicMissile | Attacks::Drain => {
                unreachable!("this effect should be done instantly")
            }
            Attacks::Shield => {
                if *count == 0 {
                    // effect is done
                    me.armor -= 7;
                }
            }
            Attacks::Poison => {
                boss.hit_points = boss.hit_points.saturating_sub(3);
                if boss.hit_points == 0 {
                    // we have a winning path
                    return Some(*mana);
                }
            }
            Attacks::Recharge => {
                me.mana += 101;
            }
        }
    }
    None
}

fn minimax(
    mut me: Person,
    mut boss: Person,
    mut mana: usize,
    depth: usize,
    mut active: [usize; Attacks::COUNT],
) -> Option<usize> {
    // we got a winner
    if let Some(mana) = pre_effects(&mut me, &mut boss, &mut mana, &mut active) {
        return Some(mana);
    }

    if depth % 2 == 1 {
        // boss
        me.hit_points = me
            .hit_points
            .saturating_sub(std::cmp::max(boss.damage.saturating_sub(me.armor), 1));
        // this is a loosing path
        if me.hit_points == 0 {
            None
        } else {
            // simple recursion no branches needed for boss move
            minimax(me, boss, mana, depth + 1, active)
        }
    } else {
        // mine
        let mut min = None;
        for (id, att, cost, durr) in Attacks::iter()
            .map(|att| (att as usize, att, att.mana(), att.duration()))
            // check if effect is active => don't try it out
            .filter(|(_, _, cost, _)| *cost > 0)
        {
            let mut me = me.clone();
            let mut boss = boss.clone();
            let mut mana = mana.clone();
            let mut active = active.clone();

            // set the duration of the effect
            active[id] = durr;

            // check if there is enough mana for effect
            if cost >= me.mana {
                // no path
                continue;
            }

            me.mana -= cost;
            mana += cost;

            // check if new path will be cheaper then previous winning ones
            // (pruning step)
            if let Some(other) = min {
                if other > mana {
                    continue;
                }
            }

            let mut check_win = |mana: usize| {
                // found a winning path
                let t = if let Some(other) = min {
                    std::cmp::min(other, mana)
                } else {
                    mana
                };
                min = Some(t);
            };

            let mut do_damage = |damage: usize| {
                // do the damage
                boss.hit_points = boss.hit_points.saturating_sub(damage);
                if boss.hit_points == 0 {
                    // winning path, so check if better score
                    check_win(mana);
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
                    me.hit_points += 2;
                }
                Attacks::Shield => {
                    me.armor += 7;
                }
                Attacks::Poison | Attacks::Recharge => {
                    // no direct effect
                }
            }

            // try this path
            if let Some(res) = minimax(me, boss, mana, depth + 1, active) {
                check_win(res);
            }
        }
        min
    }
}

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
    fn mana(&self) -> usize {
        match self {
            Attacks::MagicMissile => 53,
            Attacks::Drain => 73,
            Attacks::Shield => 113,
            Attacks::Poison => 173,
            Attacks::Recharge => 229,
        }
    }

    fn duration(&self) -> usize {
        match self {
            Attacks::MagicMissile => 0,
            Attacks::Drain => 0,
            Attacks::Shield => 6,
            Attacks::Poison => 6,
            Attacks::Recharge => 5,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Person {
    pub hit_points: usize,
    pub damage: usize,
    pub armor: usize,
    pub mana: usize,
}

pub fn input() -> Option<Person> {
    let mut it = INPUT.lines().filter_map(|s| {
        let (_, v) = s.rsplit_once(": ")?;
        v.parse().ok()
    });

    Some(Person {
        hit_points: it.next()?,
        damage: it.next()?,
        mana: 0,
        armor: 0,
    })
}
