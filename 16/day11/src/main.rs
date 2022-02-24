use floorplan::*;
use std::collections::{HashSet, VecDeque};
const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("result <{}>", one());
}

fn one() -> usize {
    let fp = parse(INPUT).unwrap();
    println!("{:#?}", fp);
    run(fp).unwrap()
}

fn run(fp: FloorPlan) -> Option<usize> {
    let mut queue = VecDeque::new();
    queue.push_back((0, fp));

    let mut min = None;

    while let Some(fp) = queue.pop_back() {
        todo!()
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
            Type::Microchip(_) => self.clone(),
            Type::Generator(name) => Type::Microchip(name),
        }
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

pub enum Movement<'name> {
    Single(Type<'name>),
    Multiple(Type<'name>, Type<'name>),
}

mod floorplan {
    use super::*;

    const FLOORS: usize = 4;

    #[derive(Debug, Clone)]
    struct Floor<'name> {
        /// Stores the single generators
        single_gen: HashSet<Type<'name>>,
        /// Stores the single chips
        single_chip: HashSet<Type<'name>>,
        /// Stores all the pairs
        pair: HashSet<Type<'name>>,
    }

    #[derive(Debug, Clone)]
    pub struct FloorPlan<'name> {
        floors: [Floor<'name>; FLOORS],
        size: usize,
    }

    impl<'name> FloorPlan<'name> {
        pub fn finished(&self) -> bool {
            self.floors[FLOORS - 1].pair.len() == self.len() / 2
        }

        /// Will transfer the given
        pub fn transfer(&mut self, from: usize, to: usize, moves: Movement<'name>) -> Option<()> {
            if !self.is_legal(from, to, &moves) {
                return None;
            }

            Some(())
        }

        /// Will only check if the move is legal or not.
        fn is_legal(&self, from: usize, to: usize, moves: &Movement<'name>) -> bool {
            // check next floor
            match moves {
                Movement::Single(a) => {
                    match a {
                        Type::Microchip(_) => {
                            // check if fitting generator is on new floor
                            // or that there currently is no free generator
                            // there
                            // An RTG powering a microchip is still dangerous
                            // to other microchips.
                            let new = self.floors[to].pair.is_empty()
                                && (self.floors[to].single_gen.is_empty()
                                    || self.floors[to].single_gen.contains(&a.compatible()));

                            if !new {
                                return false;
                            }
                            // check that the state of the old floor will not
                            // be illegal now that the chip is gone, so
                            // that the now single generator will not
                            // destroy the other chips
                            let old = self.floors[from].pair.len() > 1
                                && (self.floors[from].pair.contains(&a.general())
                                    && self.floors[from].single_chip.len() > 0);

                            old
                        }
                        Type::Generator(_) => {
                            // same process as for the chip, just now
                            // for the generator

                            // new floor
                            let new = self.floors[to].single_chip.is_empty()
                                || self.floors[to].single_chip.contains(&a.compatible());

                            if !new {
                                return false;
                            }

                            // old floor
                            let old = self.floors[from].pair.contains(&a.general())
                                && self.floors[from].single_gen.len() > 0;

                            old
                        }
                    }
                }
                Movement::Multiple(a, b) => {
                    match (a, b) {
                        (Type::Microchip(m), Type::Generator(g))
                        | (Type::Generator(g), Type::Microchip(m)) => {
                            // if they are the same type, we can just
                            // move them
                            // An RTG powering a microchip is still
                            // dangerous to other microchips.
                            m == g && self.floors[to].single_chip.len() > 0
                        }

                        (a, b) => {
                            // we can check here both generators and microchip
                            // at the same time
                            //
                            // if one of the two are legal, then we know
                            // that either the next floor is empty or
                            // that one of them has activated a generator -
                            // and we know that the previous floor is okay
                            // then as well.
                            self.is_legal(from, to, &Movement::Single(a.clone()))
                                || self.is_legal(from, to, &Movement::Single(b.clone()))
                        }
                    }
                }
            }
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

    impl<'name> Default for FloorPlan<'name> {
        fn default() -> Self {
            // SAFETY: used unsafe here according to the module documentation
            // https://doc.rust-lang.org/stable/std/mem/union.MaybeUninit.html#initializing-an-array-element-by-element

            use std::mem::{self, MaybeUninit};
            let mut floors: [MaybeUninit<HashSet<Type<'name>>>; FLOORS] =
                unsafe { MaybeUninit::uninit().assume_init() };

            for f in &mut floors {
                f.write(HashSet::with_capacity(FLOORS * 2));
            }

            // Everything is initialized. Transmute the array to the
            // initialized type.
            Self {
                floors: unsafe { mem::transmute::<_, _>(floors) },
                size: 0,
            }
        }
    }
}
