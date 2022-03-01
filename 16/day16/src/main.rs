const INPUT: &str = "01111010110010011";

fn main() {
    println!("result {:?}", two());
}

fn one() -> String {
    const MAX_LEN: usize = 272;
    inner(INPUT, MAX_LEN)
}

fn two() -> String {
    const MAX_LEN: usize = 35651584;
    inner(INPUT, MAX_LEN)
}

fn inner(input: &str, max_len: usize) -> String {
    let mut a: Vec<_> = input.chars().collect();
    a.reserve(max_len);
    let mut b = Vec::with_capacity(a.capacity());

    // inc
    while a.len() < max_len {
        b.clear();
        for c in a.iter().rev() {
            let t = match c {
                '1' => '0',
                '0' => '1',
                _ => unimplemented!(),
            };

            b.push(t);
        }

        a.push('0');
        a.append(&mut b);
    }

    a.truncate(max_len);

    while a.len() % 2 == 0 {
        b.clear();
        for c in a.chunks_exact(2) {
            let f = match c {
                ['0', '0'] | ['1', '1'] => '1',
                _ => '0',
            };
            b.push(f);
        }
        std::mem::swap(&mut a, &mut b);
    }

    a.into_iter().collect()
}
