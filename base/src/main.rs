use match_util::*;
use pub_lib;

fn main() {
    println!("{}", 98_001);
    println!("{}", 0x11);
    println!("{}", 0o10);
    println!("{}", 0b0000_0010);
    println!("{}", b'A');
    let x = '中';
    println!("字符'中'占用了{}字节的内存大小", std::mem::size_of_val(&x));

    let mut v = vec![1, 2, 3, 4, 5];
    v.push(1);
    println!("{:?}", v);

    for item in &v {
        println!("{}", item);
    }
    let i = 0;
    let j = 3;
    v[i] ^= v[j];
    v[j] ^= v[i];
    v[i] ^= v[j];
    println!("{:?}", v);

    let number = if true {
        5
    } else {
        6
    };
    println!("Number is {}", number);

    let d1 = Direction::East;
    let d2 = Direction::West;
    let d3 = Direction::North;
    let d4 = Direction::South;
    direction_match(d1);
    direction_match(d2);
    direction_match(d3);
    direction_match(d4);
}
