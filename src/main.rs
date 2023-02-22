fn main() {
    let x = gcd(8, 4);
    println!("{}", x);
}

pub fn gcd(a: u128, b: u128) -> u128 {
    let mut r0: u128 = a;
    let mut r1: u128 = b;

    if a > b {
        r0 = b;
        r1 = a;
    }

    let mut r2: u128 = 1;

    while r2 > 0 {
        r2 = r1.rem_euclid(r0);
        r1 = r0;
        r0 = r2;
    }

    r1
}
