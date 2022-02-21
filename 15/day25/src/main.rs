const COORD: (usize, usize) = (2981, 3075);

fn main() {
    println!("result <{}>", one());
}

fn one() -> usize {
    calc(COORD)
}

fn calc((grow, gcolumn): (usize, usize)) -> usize {
    const START: usize = 20151125;
    let mut curr = START;

    if let (1, 1) = (grow, gcolumn) {
        return curr;
    }

    // outer loop
    let mut id = 0;
    'outer: for col in 1.. {
        for row in (1..=col).rev() {
            id += 1;
            if let (1, 1) = (col, row) {
                continue;
            }

            curr = (curr * 252533) % 33554393;
            if row == grow && (col + 1 - row) == gcolumn {
                eprintln!("row <{row}> - col <{col}> => id <{id}> - val : <{curr}>");
                break 'outer;
            }
        }
    }
    curr
}

#[test]
fn test_one() {
    assert_eq!(30943339, calc((1, 4)))
}
