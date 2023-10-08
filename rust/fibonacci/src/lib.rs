pub fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let mut tot = 1;
    let mut previus = 0;
    let mut index = 1;

    while index < n {
        index += 1;
        let old_tot = tot;
        tot += previus;
        previus = old_tot;
    }

    return tot;
}
