use std::{collections::HashMap, hash::Hash};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("result <{}>", one());
    println!("result <{}>", two());
}

fn one() -> usize {
    let coms = parse(INPUT).expect("parsing error");

    let mut asm = Assembly::default();

    for com in &coms {
        let _ = process_com(com.clone(), &mut asm);
    }

    assert!(asm.pending.len() == 0, "not finished processing");

    // Based on your instructions, what is the number of the bot that is
    // responsible for comparing value-61 microchips with value-17 microchips?
    //
    // search the Assembly for the robot that used the given chips
    let mut res = 0;
    for (id, bot) in &asm.bots {
        if let (17, 61) = (bot.low(), bot.high()) {
            res = *id;
            break;
        }
    }
    res
}

fn two() -> usize {
    let coms = parse(INPUT).expect("parsing error");

    let mut asm = Assembly::default();

    for com in &coms {
        let _ = process_com(com.clone(), &mut asm);
    }

    assert!(asm.pending.len() == 0, "not finished processing");

    // What do you get if you multiply together the values of one
    // chip in each of outputs 0, 1, and 2?

    asm.output[&0] * asm.output[&1] * asm.output[&2]
}

fn finish_process(bot_id: BotID, asm: &mut Assembly) -> Option<()> {
    // run command of the bot now that it is ready
    // assuming, there is one
    let com = asm.pending.remove(&bot_id)?;

    let bot = asm.bots.get(&bot_id)?;

    // process both high and low if possible
    for (bot, value) in [(&com.low, bot.low()), (&com.high, bot.high())] {
        match bot {
            Target::Output(id) => {
                assert!(!asm.output.contains_key(id));
                asm.output.insert(*id, value);
            }
            Target::Bot(id) => {
                // we don't care about the Option here
                let _ = process_com(Command::Value(Value { value, to: *id }), asm);
            }
        }
    }

    // we done boys
    asm.done.insert(bot_id, com);
    Some(())
}

fn process_val(val: Value, asm: &mut Assembly) -> Option<()> {
    // needs to add the value directly to the bot
    if asm.push(val.clone()) {
        // try to fill up bot information, that are now
        // possible
        finish_process(val.to, asm)
    } else {
        None
    }
}

fn process_bot(bot: BotCommand, asm: &mut Assembly) -> Option<()> {
    let bot_id = bot.from;
    assert!(
        !asm.pending.contains_key(&bot_id),
        "bot has already been added"
    );

    asm.pending.insert(bot_id, bot);

    let runnable = {
        // workaround the mutability issue
        if let Some(bot) = asm.bots.get(&bot_id) {
            // we can run the bot
            bot.0.len() == 2
        } else {
            false
        }
    };

    if runnable {
        finish_process(bot_id, asm)
    } else {
        Some(())
    }
}

fn process_com(com: Command, asm: &mut Assembly) -> Option<()> {
    match com {
        Command::Value(val) => process_val(val, asm),
        Command::Bot(bot) => process_bot(bot, asm),
    }
}

fn parse(input: &str) -> Option<Vec<Command>> {
    let mut coms = Vec::new();

    for line in input.lines().map(str::trim).filter(|a| !a.is_empty()) {
        coms.push(parse_line(line)?);
    }
    Some(coms)
}

fn parse_line(line: &str) -> Option<Command> {
    let is = line.split(" ").next()?;
    let add = match is {
        "value" => {
            let value = line.split(" ").skip(1).next()?.parse().ok()?;
            let to = line.rsplit(" ").next()?.parse().ok()?;
            Command::Value(Value { value, to })
        }
        "bot" => {
            let from = line.split(" ").skip(1).next()?.parse().ok()?;

            let get = |to, id: &str| {
                let id: BotID = id.parse().ok()?;
                Some(match to {
                    "bot" => Target::Bot(id),
                    "output" => Target::Output(id),
                    _ => unreachable!(),
                })
            };

            let get_it = |c| {
                let mut it = line.rsplit(" ").skip(c);
                let n = it.next();
                get(it.next()?, n?)
            };

            let low = get_it(5)?;

            let high = get_it(0)?;

            Command::Bot(BotCommand { from, high, low })
        }
        _ => unreachable!(),
    };

    Some(add)
}

#[derive(Debug, Default)]
struct Assembly {
    /// Stores the output information
    output: HashMap<BotID, ChipID>,
    /// Stores all the bots
    bots: HashMap<BotID, Bot>,
    /// Stores the bot commands that are waiting at least one input chip
    pending: HashMap<BotID, BotCommand>,
    /// Stores the bot commands that are have finished processing
    done: HashMap<BotID, BotCommand>,
}

impl Assembly {
    fn push(&mut self, com: Value) -> bool {
        let bot = self.bots.entry(com.to).or_default();
        bot.0.push(com.value);
        bot.0.len() == 2
    }
}

type ChipID = usize;
type BotID = usize;

#[derive(Debug, Clone)]
enum Target {
    Output(BotID),
    Bot(BotID),
}

#[derive(Debug, Default)]
struct Bot(tinyvec::ArrayVec<[ChipID; 2]>);

impl Bot {
    fn low(&self) -> ChipID {
        std::cmp::min(self.0[0], self.0[1])
    }

    fn high(&self) -> ChipID {
        std::cmp::max(self.0[0], self.0[1])
    }
}

#[derive(Debug, Clone)]
struct BotCommand {
    from: BotID,
    high: Target,
    low: Target,
}

#[derive(Debug, Clone)]
struct Value {
    value: ChipID,
    to: BotID,
}

#[derive(Debug, Clone)]
enum Command {
    Value(Value),
    Bot(BotCommand),
}
