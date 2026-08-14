// If it's odd, multiply it by 3 and add 1.
// If it's even, divide it by 2.

pub fn collatz(n: u64) -> Option<u64> {
    let mut n = n;
    if n == 0 {
        return None;
    }
    let mut res = 0;

    while n != 1 {
        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = n * 3 + 1
        }
        res += 1;
    }

    Some(res)
}
