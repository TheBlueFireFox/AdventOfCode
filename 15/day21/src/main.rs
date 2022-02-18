#![allow(dead_code)]
use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");
const ITEMS: &str = include_str!("../input_shop.txt");

fn main() {
    println!("result <{}>", two());
}

fn one() -> f32 {
    let shop = parse::shop().unwrap();
    let boss = parse::input().unwrap();
    let mut me = Person {
        hit_points: 100.0,
        ..Default::default()
    };

    // try all options
    let mut cost: Option<AddedItem> = None;
    for weapon in &shop.weapons {
        for armor in &shop.armor {
            for rings in shop
                .rings
                .iter()
                .permutations(2)
                // remove permuation where two times the same ring have been used
                .filter(|a| a[0] != a[1])
            {
                let current = weapon + armor + rings[0] + rings[1];
                me.armor = current.armor;
                me.damage = current.damage;

                // check if loadout can win
                let boss_rounds = Person::attacks(&boss, &me);
                let me_rounds = Person::attacks(&me, &boss);
                if boss_rounds < me_rounds {
                    continue;
                }

                if let Some(prev) = &cost {
                    if prev.cost < current.cost {
                        continue;
                    }
                }
                cost = Some(current);
            }
        }
    }

    cost.unwrap().cost
}

fn two() -> f32 {
    let shop = parse::shop().unwrap();
    let boss = parse::input().unwrap();
    let mut me = Person {
        hit_points: 100.0,
        ..Default::default()
    };

    // try all options
    let mut cost: Option<AddedItem> = None;
    for weapon in &shop.weapons {
        for armor in &shop.armor {
            for rings in shop
                .rings
                .iter()
                .permutations(2)
                // remove permuation where two times the same ring have been used
                .filter(|a| a[0] != a[1])
            {
                let current = weapon + armor + rings[0] + rings[1];
                me.armor = current.armor;
                me.damage = current.damage;

                // check if loadout can win
                let boss_rounds = Person::attacks(&boss, &me);
                let me_rounds = Person::attacks(&me, &boss);

                if boss_rounds > me_rounds {
                    continue;
                }

                if let Some(prev) = &cost {
                    if prev.cost >= current.cost {
                        continue;
                    }
                }
                cost = Some(current);
            }
        }
    }

    cost.unwrap().cost
}

#[derive(Debug, Default)]
pub struct Person {
    pub hit_points: f32,
    pub damage: f32,
    pub armor: f32,
}

impl Person {
    fn attacks(attacker: &Self, defender: &Self) -> f32 {
        let res = defender.hit_points / f32::max(attacker.damage - defender.armor, 1.0);
        res.ceil()
    }
}

#[derive(Debug, Default)]
pub struct Shop<'name> {
    pub weapons: Vec<Item<'name>>,
    pub armor: Vec<Item<'name>>,
    pub rings: Vec<Item<'name>>,
}

impl Shop<'_> {
    fn size(&self) -> usize {
        self.weapons.len() + self.armor.len() + self.rings.len()
    }
}

#[derive(Debug, Default)]
pub struct Units<'name> {
    items: Vec<Item<'name>>,
}

impl Units<'_> {
    fn size(&self) -> usize {
        self.items.len()
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Item<'name> {
    pub name: &'name str,
    pub cost: f32,
    pub damage: f32,
    pub armor: f32,
}

use impl_ops::*;
use std::ops;

pub struct AddedItem {
    cost: f32,
    damage: f32,
    armor: f32,
}

impl From<&Item<'_>> for AddedItem {
    fn from(other: &Item<'_>) -> Self {
        other.clone().into()
    }
}

impl From<Item<'_>> for AddedItem {
    fn from(other: Item<'_>) -> Self {
        Self {
            cost: other.cost,
            damage: other.damage,
            armor: other.armor,
        }
    }
}

impl_op_ex!(+ |a: &AddedItem, b: &AddedItem| -> AddedItem {
    let cost = a.cost + b.cost;
    let damage = a.damage + b.damage;
    let armor = a.armor + b.armor;
    AddedItem { cost, damage, armor}
});

impl_op_ex!(+ |a: &Item<'_>, b: &Item<'_>| -> AddedItem {
    let a: AddedItem = a.into();
    a + b
});

impl_op_ex!(+ |a: &Item<'_>, b: &AddedItem| -> AddedItem {
    b + a
});

impl_op_ex!(+ |a: &AddedItem, b: &Item<'_>| -> AddedItem {
    let b : AddedItem = b.into();
    a + b
});

mod parse {
    use super::*;

    pub fn shop() -> Option<Shop<'static>> {
        let mut shop = Shop::default();

        let mut idx = 0;

        for line in ITEMS.lines().map(str::trim) {
            if line.is_empty() {
                idx += 1;
                continue;
            }

            if let Some(item) = item(line) {
                match idx {
                    0 => shop.weapons.push(item),
                    1 => shop.armor.push(item),
                    2 => shop.rings.push(item),
                    _ => unreachable!(),
                }
            }
        }

        Some(shop)
    }

    fn item(line: &str) -> Option<Item<'_>> {
        // initial line or entry break
        if line.ends_with("Armor") || line.is_empty() {
            return None;
        }

        let parse = |v: &str| v.parse().ok();

        let (name, line) = line.split_at(12);
        let name = name.trim();

        let (cost, line) = line.split_at(8);
        let cost = cost.trim();

        let (damage, line) = line.split_at(8);
        let damage = damage.trim();

        let armor = line.trim();

        Some(Item {
            name,
            cost: parse(cost)?,
            damage: parse(damage)?,
            armor: parse(armor)?,
        })
    }

    pub fn input() -> Option<Person> {
        let mut it = INPUT.lines().filter_map(|s| {
            let (_, v) = s.rsplit_once(": ")?;
            v.parse().ok()
        });

        Some(Person {
            hit_points: it.next()?,
            damage: it.next()?,
            armor: it.next()?,
        })
    }
}
