pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => calc_fibonacci(n),
    }
}

fn calc_fibonacci(n: u32) -> u32 {
    let mut tot = 1;
    let mut previus = 0;

    for _ in 1..n {
        let old_tot = tot;
        tot += previus;
        previus = old_tot;
    }

    return tot;
}
