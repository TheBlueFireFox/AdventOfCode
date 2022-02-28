use floorplan::*;
use std::collections::{HashMap, HashSet, VecDeque};
const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("result <{}>", two());
}

fn one() -> usize {
    one_inner(INPUT)
}

fn one_inner(input: &str) -> usize {
    let fp = parse(input).unwrap();
    run(fp)
}

fn two() -> usize {
    two_inner(INPUT)
}

fn two_inner(input: &str) -> usize {
    let mut fp = parse(input).unwrap();
    let additions = ["elerium", "dilithium"];
    for a in additions {
        let a = Type::Microchip(a.into());
        fp.insert(0, a.clone());
        fp.insert(0, a.compatible());
    }
    run(fp)
}

fn run<'name>(fp: FloorPlan<'name>) -> usize {
    let mut queue = VecDeque::new();

    let mut visited_states = HashMap::new();
    let mut enqued_states = HashSet::new();

    enqued_states.insert(fp.plan());
    queue.push_back((0, fp));

    let mut min = usize::MAX;

    while let Some((steps, fp)) = queue.pop_front() {
        if cfg!(debug_assertions) {
            println!("{}", fp);
        }

        if fp.finished() {
            min = std::cmp::min(min, steps);
            continue;
        } else if steps + 1 > min {
            continue;
        }

        let plan = fp.plan();

        // possible steps
        for movement in fp.all_moves() {
            if cfg!(debug_assertions) {
                print!("\t{movement:?} ");
            }
            if fp.is_legal(&movement).is_some() {
                if cfg!(debug_assertions) {
                    print!("legal ");
                }

                // remove unneded
                match movement {
                    Movement::Single(Direction::Down, _) => {
                        let mut is = true;
                        for i in 0..fp.elevator() {
                            if !fp.is_empty(i) {
                                if cfg!(debug_assertions) {
                                    println!("X");
                                }
                                is = false;
                                break;
                            }
                        }
                        if is {
                            continue;
                        }
                    }
                    Movement::Multiple(Direction::Down, _, _) => {
                        // don't bring two items down
                        continue;
                    }
                    _ => {}
                }

                let mut fp = fp.clone();
                fp.transfer(movement);

                let plan = fp.plan();
                // check if new plan was visited befor
                let _s = if visited_states.contains_key(&plan) {
                    "(visited)"
                } else if enqued_states.contains(&plan) {
                    "(enqued)"
                } else {
                    enqued_states.insert(plan);
                    queue.push_back((steps + 1, fp));
                    "(OK)"
                };

                if cfg!(debug_assertions) {
                    print!("{_s}");
                }
            } else {
                if cfg!(debug_assertions) {
                    print!("illegal ");
                }
            }
            if cfg!(debug_assertions) {
                println!();
            }
        }

        visited_states.insert(plan, steps);
    }

    min
}

fn parse(input: &str) -> Option<FloorPlan<'_>> {
    let mut fp = FloorPlan::default();

    for (idx, line) in input
        .lines()
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .enumerate()
    {
        // The third floor contains[ a thulium-compatible microchip.]
        let (_, info) = line.rsplit_once("contains")?;
        let info = info.trim_end_matches(".");
        // a [thulium-compatible microchip]
        for entry in info
            .split(",")
            .map(str::trim)
            .map(|v| v.trim_start_matches("and ").trim_start_matches("a "))
        {
            // [strontium-compatible] [microchip]
            let (entry, is) = entry.rsplit_once(" ")?;

            // [strontium]-compatible
            let entry = entry.trim_end_matches("-compatible");

            let is = match is {
                "microchip" => Type::Microchip(entry),
                "generator" => Type::Generator(entry),
                "relevant" => continue,
                _ => unreachable!(),
            };

            fp.insert(idx, is)?;
        }
    }

    Some(fp)
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Type<'name> {
    Microchip(&'name str),
    Generator(&'name str),
}

impl Type<'_> {
    pub fn general(&self) -> Self {
        match self {
            Type::Microchip(name) | Type::Generator(name) => Type::Microchip(name),
        }
    }

    pub fn microchip(&self) -> Self {
        self.general()
    }

    pub fn generator(&self) -> Self {
        self.general().compatible()
    }

    pub fn compatible(&self) -> Self {
        match self {
            Type::Microchip(name) => Type::Generator(name),
            Type::Generator(name) => Type::Microchip(name),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Movement<'name> {
    Single(Direction, Type<'name>),
    Multiple(Direction, Type<'name>, Type<'name>),
}

use strum::IntoEnumIterator;

#[derive(Debug, Clone, Copy, strum::EnumIter)]
pub enum Direction {
    Up,
    Down,
}

mod floorplan {

    use itertools::Itertools;

    use super::*;

    const FLOORS: usize = 4;

    #[derive(Debug, Clone, Default, PartialEq, Eq)]
    pub struct Floor<'name> {
        /// Stores the single generators
        pub single_gen: HashSet<&'name str>,
        /// Stores the single chips
        pub single_chip: HashSet<&'name str>,
        /// Stores all the pairs
        pub pair: HashSet<&'name str>,
    }

    #[derive(Debug, Clone, Default, PartialEq, Eq)]
    pub struct FloorPlan<'name> {
        /// all the floors in the building
        floors: [Floor<'name>; FLOORS],
        /// where the elevator currently is
        elevator: usize,
        /// how many floors there are
        size: usize,
    }

    impl std::fmt::Display for FloorPlan<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let get = |i| {
                if self.elevator == i {
                    'E'
                } else {
                    char::from_digit(i as u32, 10).unwrap()
                }
            };
            write!(
                f,
                "FloorPlan {{\n\
                       \t{}) {:?}\n\
                       \t{}) {:?}\n\
                       \t{}) {:?}\n\
                       \t{}) {:?}\n\
                }}",
                get(0),
                self.floors[0],
                get(1),
                self.floors[1],
                get(2),
                self.floors[2],
                get(3),
                self.floors[3]
            )
        }
    }

    impl<'name> FloorPlan<'name> {
        pub fn elevator(&self) -> usize {
            self.elevator
        }

        pub fn is_empty(&self, floor: usize) -> bool {
            (self.floors[floor].single_chip.len()
                + self.floors[floor].single_gen.len()
                + self.floors[floor].pair.len())
                == 0
        }

        /// This will return all moves possible from this
        /// floor, this will not check if the move is
        /// legal or not
        pub fn all_moves(&self) -> impl Iterator<Item = Movement<'name>> + '_ {
            let floor = &self.floors[self.elevator];
            let chips = floor
                .single_chip
                .iter()
                .map(|c| {
                    Direction::iter().map(move |dir| Movement::Single(dir, Type::Microchip(c)))
                })
                .flatten();

            let chips_mul = floor
                .single_chip
                .iter()
                .combinations(2)
                .map(|c| {
                    Direction::iter().map(move |dir| {
                        Movement::Multiple(dir, Type::Microchip(c[0]), Type::Microchip(c[1]))
                    })
                })
                .flatten();

            let gens = floor
                .single_gen
                .iter()
                .map(|c| {
                    Direction::iter().map(move |dir| Movement::Single(dir, Type::Generator(c)))
                })
                .flatten();

            let gens_mul = floor
                .single_gen
                .iter()
                .combinations(2)
                .map(|c| {
                    Direction::iter().map(move |dir| {
                        Movement::Multiple(dir, Type::Generator(c[0]), Type::Generator(c[1]))
                    })
                })
                .flatten();

            let either = floor
                .pair
                .iter()
                .map(|c| {
                    Direction::iter()
                        .map(move |dir| {
                            [
                                Movement::Single(dir, Type::Microchip(c)),
                                Movement::Single(dir, Type::Generator(c)),
                                Movement::Multiple(
                                    dir,
                                    Type::Microchip(c.clone()),
                                    Type::Generator(c.clone()),
                                ),
                            ]
                        })
                        .flatten()
                })
                .flatten();

            either
                .chain(gens_mul)
                .chain(chips_mul)
                .chain(chips)
                .chain(gens)
        }

        pub fn finished(&self) -> bool {
            self.floors[FLOORS - 1].pair.len() == self.len() / 2
        }

        pub fn plan(&self) -> (u8, [(u8, u8, u8); FLOORS]) {
            let mut floors: [_; FLOORS] = Default::default();
            for (o, f) in std::iter::zip(&self.floors, &mut floors[..]) {
                *f = (
                    o.single_chip.len() as _,
                    o.single_gen.len() as _,
                    o.pair.len() as _,
                );
            }

            (self.elevator as _, floors)
        }

        /// This is unsafe as it doesn't check if the transfer
        /// goes against the transfer rules - this does not
        /// change the elevator position but will return the
        /// next floor id.
        /// This might panic if the direction is incorrect
        unsafe fn transfer_element(&mut self, dir: Direction, thing: &Type<'name>) -> usize {
            let next = self.direction(dir);
            let (from, to) = next.expect("the direction should have been legal");

            match thing {
                Type::Microchip(thing) => {
                    let thing = thing.clone();
                    // moving from this floor
                    if !self.floors[from].single_chip.remove(&thing) {
                        self.floors[from].pair.remove(&thing);
                        self.floors[from].single_gen.insert(thing.clone());
                    };

                    // moving here
                    if self.floors[to].single_gen.remove(&thing) {
                        self.floors[to].pair.insert(thing);
                    } else {
                        self.floors[to].single_chip.insert(thing);
                    }
                }
                Type::Generator(thing) => {
                    let thing = thing.clone();
                    // moving from here
                    if !self.floors[from].single_gen.remove(&thing) {
                        self.floors[from].pair.remove(&thing);
                        self.floors[from].single_chip.insert(thing.clone());
                    };

                    // to here
                    if self.floors[to].single_chip.remove(&thing.clone()) {
                        self.floors[to].pair.insert(thing.clone());
                    } else {
                        self.floors[to].single_gen.insert(thing.clone());
                    }
                }
            }
            to
        }

        /// Will transfer the given
        pub fn transfer(&mut self, moves: Movement<'name>) -> Option<()> {
            self.is_legal(&moves)?;
            // SAFETY: we checked if the given move is legal above this
            // line, so transfering does not go against the rules given
            let to = unsafe {
                match moves {
                    Movement::Single(dir, ref thing) => self.transfer_element(dir, thing),
                    Movement::Multiple(dir, ref one, ref two) => {
                        self.transfer_element(dir, one);
                        self.transfer_element(dir, two)
                    }
                }
            };
            self.elevator = to;
            Some(())
        }

        fn direction(&self, dir: Direction) -> Option<(usize, usize)> {
            let res = match dir {
                Direction::Up => {
                    if self.elevator + 1 >= FLOORS {
                        return None;
                    }
                    (self.elevator, self.elevator + 1)
                }
                Direction::Down => (self.elevator, self.elevator.checked_sub(1)?),
            };
            Some(res)
        }

        /// Will only check if the move is legal or not.
        pub fn is_legal(&self, moves: &Movement<'_>) -> Option<()> {
            let map_floor = |(a, b)| (&self.floors[a], &self.floors[b]);

            // check next floor
            match moves {
                Movement::Single(dir, a) => {
                    let (from, to) = self.direction(*dir)?;
                    let (from_floor, to_floor) = map_floor((from, to));

                    match a {
                        Type::Microchip(a) => {
                            // check if fitting generator is on new floor
                            // or that there currently is no free generator
                            // there
                            // An RTG powering a microchip is still dangerous
                            // to other microchips.
                            let new = (to_floor.pair.is_empty() && to_floor.single_gen.is_empty())
                                || to_floor.single_gen.contains(a);

                            // check that the state of the old floor will not
                            // be illegal now that the chip is gone, so
                            // that the now possibly single generator will not
                            // destroy the other chips
                            //
                            // moving a chip will not do much in the old floor
                            // (max a single free generator)
                            let old = true;

                            new && old
                        }
                        Type::Generator(a) => {
                            // same process as for the chip, just now
                            // for the generator

                            // new floor
                            let new =
                                to_floor.single_chip.is_empty() || to_floor.single_chip.contains(a);

                            // old floor
                            let old = !(from_floor.pair.contains(a)
                                && (from_floor.single_gen.len() > 0 || from_floor.pair.len() > 1));

                            new && old
                        }
                    }
                }
                Movement::Multiple(dir, a, b) => {
                    let (from, to) = self.direction(*dir)?;
                    let (from_floor, to_floor) = map_floor((from, to));

                    match (a, b) {
                        (Type::Microchip(m), Type::Generator(g))
                        | (Type::Generator(g), Type::Microchip(m)) => {
                            // if they are the same type, we can just
                            // move them; otherwise we are not allowed to
                            // move them
                            m == g
                        }
                        (Type::Microchip(a), Type::Microchip(b)) => {
                            // we can only move the chips, if the next floor
                            // has no generators or both activate their own gens
                            //
                            // An RTG powering a microchip is still dangerous
                            // to other microchips.
                            let new = match to_floor.single_gen.len() {
                                0 => to_floor.pair.is_empty(),
                                1 => false,
                                _ => {
                                    to_floor.single_gen.contains(a)
                                        && to_floor.single_gen.contains(b)
                                }
                            };

                            // as this situation can only happen from single_chip
                            // we don't have to check the old floor
                            let old = true;

                            new && old
                        }
                        (Type::Generator(a), Type::Generator(b)) => {
                            // we can check here both generators
                            // at the same time
                            //
                            // if one of the two are legal, then we know
                            // that either the next floor is empty or
                            // that one of them has activated a microchip -
                            // without there beeing an other one left
                            let new = match to_floor.single_chip.len() {
                                0 => true,
                                1 => {
                                    to_floor.single_chip.contains(a)
                                        || to_floor.single_chip.contains(b)
                                }
                                2 => {
                                    to_floor.single_chip.contains(a)
                                        && to_floor.single_chip.contains(b)
                                }
                                _ => false,
                            };

                            let old = match from_floor.pair.len() {
                                0 => true,
                                1 => {
                                    from_floor.single_gen.len() == 0
                                        && (from_floor.pair.contains(a)
                                            || from_floor.pair.contains(b))
                                }
                                2 => {
                                    from_floor.single_gen.len() == 0
                                        && from_floor.pair.contains(a)
                                        && from_floor.pair.contains(b)
                                }
                                _ => false,
                            };

                            new && old
                        }
                    }
                }
            }
            .then(|| ())
        }

        /// Will blindly insert the information into the floor plan
        /// as I assume that the values can be of unfitting order and
        /// that the input is alredy in a valid state.
        pub fn insert(&mut self, idx: usize, value: Type<'name>) -> Option<()> {
            if let Some(floor) = self.floors.get_mut(idx) {
                match value {
                    Type::Microchip(value) => {
                        // check if value has a pair
                        if floor.single_gen.remove(&value) {
                            // using the general representation for
                            // consistency
                            floor.pair.insert(value);
                        } else {
                            floor.single_chip.insert(value);
                        }
                    }
                    Type::Generator(value) => {
                        // check if value has a pair
                        if floor.single_chip.remove(&value) {
                            // using the general representation for
                            // consistency
                            floor.pair.insert(value);
                        } else {
                            floor.single_gen.insert(value);
                        }
                    }
                }
                self.size += 1;
                Some(())
            } else {
                None
            }
        }

        pub fn len(&self) -> usize {
            self.size
        }
    }
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &str = include_str!("../input_test.txt");

    #[test]
    fn one() {
        assert_eq!(11, super::one_inner(TEST_INPUT));
    }
}
