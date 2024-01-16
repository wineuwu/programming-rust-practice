use std::env;
use std::str::FromStr;

fn main() {
    //建立一個可變的空向量
    let mut numbers = Vec::new();

    // 透過 for 迴圈來取得命令列參數 env::args() 會回傳一個迭代器 skip 會跳過第一個參數
    for arg in env::args().skip(1) {
        // 將字串轉換成 u64 並加入到 numbers 向量中, borrow arg 並透過 &arg 傳遞給 from_str
        //expect 會在轉換失敗時產生一個錯誤訊息
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        eprint!("Usage: gcd NUMBER ...");
        // 強制結束程式
        std::process::exit(1);
    }

    let mut d = numbers[0];

    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}
