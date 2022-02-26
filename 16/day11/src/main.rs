use floorplan::*;
use std::collections::{HashMap, HashSet, VecDeque};
const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("result <{}>", one());
}

fn one() -> usize {
    one_inner(INPUT)
}

fn one_inner(input: &str) -> usize {
    let fp = parse(input).unwrap();
    println!("{:#?}", fp);
    run(&fp)
}

fn run(fp: &FloorPlan<'_>) -> usize {
    let mut queue = VecDeque::new();

    let mut visited_states = HashMap::new();
    let mut enqued_states = HashSet::new();

    enqued_states.insert(fp.plan());
    queue.push_back((0, fp.clone()));

    let mut min = usize::MAX;

    while let Some((steps, fp)) = queue.pop_front() {
        if fp.finished() {
            min = std::cmp::min(min, steps);
            continue;
        } else if steps + 1 > min {
            continue;
        }

        // // possible steps
        for movement in fp.all_moves() {
            if fp.is_legal(&movement).is_some() {
                let mut fp = fp.clone();
                fp.transfer(movement);
                // check if new plan was visited befor
                if visited_states.contains_key(&fp.plan()) || enqued_states.contains(&fp.plan()) {
                    continue;
                }
                queue.push_back((steps + 1, fp));
            }
        }
        // // chips
        // for c in &floor.single_chip {
        //     for dir in Direction::iter() {
        //         let movement = Movement::Single(dir, c.clone());
        //         let mut fp = fp.clone();
        //         if fp.transfer(movement).is_some() {
        //             //         // check if movement is valid
        //             //             fp.transfer(movement);

        //             //             // check if new plan was visited befor
        //             //             if visited_states.contains_key(&fp.plan()) || enqued_states.contains(&fp.plan())
        //             //             {
        //             //                 continue;
        //             //             }
        //             queue.push_back((steps + 1, fp));
        //         }
        //     }
        // }

        enqued_states.remove(&fp.plan());
        visited_states.insert(fp.plan(), steps);
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

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
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

    pub fn name(&self) -> &str {
        match self {
            Type::Microchip(name) | Type::Generator(name) => name,
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
    use super::*;

    const FLOORS: usize = 4;

    #[derive(Debug, Clone, Default)]
    pub struct Floor<'name> {
        /// Stores the single generators
        pub single_gen: HashSet<Type<'name>>,
        /// Stores the single chips
        pub single_chip: HashSet<Type<'name>>,
        /// Stores all the pairs
        pub pair: HashSet<Type<'name>>,
    }

    #[derive(Debug, Clone, Default)]
    pub struct FloorPlan<'name> {
        /// all the floors in the building
        floors: [Floor<'name>; FLOORS],
        /// where the elevator currently is
        elevator: usize,
        /// how many floors there are
        size: usize,
    }

    impl<'name> FloorPlan<'name> {
        pub fn inners(&self) -> Floor<'name> {
            self.floors[self.elevator].clone()
        }

        /// This will return all moves possible from this
        /// floor
        pub fn all_moves(&self) -> impl Iterator<Item = Movement<'_>> {
            let floor = &self.floors[self.elevator];
            let chips = floor
                .single_chip
                .iter()
                .map(|c| Direction::iter().map(move |dir| Movement::Single(dir, c.clone())))
                .flatten();

            let gens = floor
                .single_gen
                .iter()
                .map(|c| Direction::iter().map(move |dir| Movement::Single(dir, c.clone())))
                .flatten();

            let either = floor
                .pair
                .iter()
                .map(|c| {
                    Direction::iter()
                        .map(move |dir| {
                            [
                                Movement::Single(dir, c.microchip()),
                                Movement::Single(dir, c.generator()),
                                Movement::Multiple(dir, c.microchip(), c.generator()),
                            ]
                        })
                        .flatten()
                })
                .flatten();

            chips
                .chain(gens)
                .chain(either)
                .filter(|e| self.is_legal(e).is_some())
        }

        pub fn plan(&self) -> [(usize, usize); FLOORS] {
            let mut floors = [(0, 0); FLOORS];

            for (idx, f) in self.floors.iter().enumerate() {
                let chip = f.single_chip.len() + f.pair.len();
                let gen = f.single_gen.len() + f.pair.len();

                floors[idx] = (chip, gen);
            }
            floors
        }

        pub fn finished(&self) -> bool {
            self.floors[FLOORS - 1].pair.len() == self.len() / 2
        }

        /// This is unsafe as it doesn't check if the transfer
        /// goes against the transfer rules
        /// This might panic if the direction is incorrect
        unsafe fn transfer_element(&mut self, dir: Direction, thing: &Type<'name>) {
            let (from, to) = self
                .direction(dir)
                .expect("the direction should have been legal");

            match thing {
                Type::Microchip(_) => {
                    if !self.floors[from].single_chip.remove(thing) {
                        self.floors[from].pair.remove(&thing.general());
                        self.floors[from].single_gen.insert(thing.compatible());
                    };

                    if self.floors[to].single_gen.remove(&thing.compatible()) {
                        self.floors[to].pair.insert(thing.general());
                    } else {
                        self.floors[to].single_chip.insert(thing.clone());
                    }
                }
                Type::Generator(_) => {
                    if !self.floors[from].single_gen.remove(thing) {
                        self.floors[from].pair.remove(thing);
                        self.floors[from].single_chip.insert(thing.compatible());
                    };

                    if self.floors[to].single_chip.remove(&thing.compatible()) {
                        self.floors[to].pair.insert(thing.general());
                    } else {
                        self.floors[to].single_chip.insert(thing.clone());
                    }
                }
            }
        }

        /// Will transfer the given
        pub fn transfer(&mut self, moves: Movement<'name>) -> Option<()> {
            self.is_legal(&moves)?;
            // SAFETY: we checked if the given move is legal above this
            // line, so transfering does not go against the rules given
            unsafe {
                match moves {
                    Movement::Single(dir, ref thing) => self.transfer_element(dir, thing),
                    Movement::Multiple(dir, ref one, ref two) => {
                        self.transfer_element(dir, one);
                        self.transfer_element(dir, two);
                    }
                }
            }
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
        pub fn is_legal(&self, moves: &Movement<'name>) -> Option<()> {
            let map_floor = |(from, to)| (&self.floors[from], &self.floors[to]);

            // check next floor
            match moves {
                Movement::Single(dir, a) => {
                    let (to, from) = self.direction(*dir)?;
                    let (to_floor, from_floor) = map_floor((to, from));

                    match a {
                        Type::Microchip(_) => {
                            // check if fitting generator is on new floor
                            // or that there currently is no free generator
                            // there
                            // An RTG powering a microchip is still dangerous
                            // to other microchips.
                            let new = to_floor.pair.is_empty()
                                && (to_floor.single_gen.is_empty()
                                    || to_floor.single_gen.contains(&a.compatible()));

                            // check that the state of the old floor will not
                            // be illegal now that the chip is gone, so
                            // that the now possibly single generator will not
                            // destroy the other chips
                            let old = from_floor.pair.len() > 1
                                && (from_floor.pair.contains(&a.general())
                                    && from_floor.single_chip.len() > 0);

                            new && old
                        }
                        Type::Generator(_) => {
                            // same process as for the chip, just now
                            // for the generator

                            // new floor
                            let new = to_floor.single_chip.is_empty()
                                || to_floor.single_chip.contains(&a.compatible());

                            // old floor
                            let old = from_floor.pair.contains(&a.general())
                                && from_floor.single_gen.len() > 0;

                            new && old
                        }
                    }
                }
                Movement::Multiple(dir, a, b) => {
                    let (to, from) = self.direction(*dir)?;
                    let (to_floor, _) = map_floor((to, from));

                    match (a, b) {
                        (Type::Microchip(m), Type::Generator(g))
                        | (Type::Generator(g), Type::Microchip(m)) => {
                            // if they are the same type, we can just
                            // move them
                            // An RTG powering a microchip is still
                            // dangerous to other microchips.
                            m == g && to_floor.single_chip.len() > 0
                        }
                        (a, b) => {
                            // we can check here both generators and microchip
                            // at the same time
                            //
                            // if one of the two are legal, then we know
                            // that either the next floor is empty or
                            // that one of them has activated a microchip -
                            // and we know that the previous floor is okay
                            // then as well.
                            self.is_legal(&Movement::Single(*dir, a.clone())).is_some()
                                || self.is_legal(&Movement::Single(*dir, b.clone())).is_some()
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
                match value.clone() {
                    Type::Microchip(_) => {
                        // check if value has a pair
                        if floor.single_gen.remove(&value.compatible()) {
                            floor.pair.insert(value);
                        } else {
                            floor.single_chip.insert(value);
                        }
                    }
                    Type::Generator(_) => {
                        // check if value has a pair
                        if floor.single_chip.remove(&value.compatible()) {
                            floor.pair.insert(value);
                        } else {
                            // using the general representation for
                            // consistency
                            floor.single_chip.insert(value.general());
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
