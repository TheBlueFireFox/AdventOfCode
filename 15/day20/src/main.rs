const NEEDED: u64 = 36000000;

fn main() {
    println!("result <{}>", one());
}

fn one() -> u64 {
    one_inner(NEEDED)
}

fn one_inner(needed: u64) -> u64 {
    // search upper => more then exp
    let mut count = 1;
    let mut house = 1;
    while count < needed {
        house *= 2; // make large steps
        count = get_house_presents(house);
    }

    println!("upper house <{}>", house);

    // find exact via binanry search approach
    // if mid is too low then upper is still house
    // else mid will be upper repeate until mid
    // and low are equal or low is larger then mid
    let mut top = house;
    let mut low = house / 2;
    let mut last = None;
    while low <= top {
        let mid = top - (top - low) / 2;
        let curr = get_house_presents(mid);

        if let Some(v) = last {
            if v == curr {
                break;
            }
        }

        last = Some(curr);

        if curr < needed {
            low = mid;
        } else if curr > needed {
            top = mid;
        } else {
            // we found it
            return mid;
        }

        println!(
            "current: <{curr}>, state: <{}>, top: {top}, mid: {mid}, low: {low}",
            curr < needed
        );
    }

    top
}

fn get_house_presents(house: u64) -> u64 {
    let mut current = 0;
    for elf_nr in 1..=house {
        if house % elf_nr == 0 {
            current += elf_nr * 10;
        }
    }
    current
}

#[test]
fn test_house() {
    assert_eq!(150, get_house_presents(8));
    assert_eq!(130, get_house_presents(9));
}
