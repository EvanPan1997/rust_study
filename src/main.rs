fn main() {
    println!("{}", 98_001);
    println!("{}", 0x11);
    println!("{}", 0o10);
    println!("{}", 0b0000_0010);
    println!("{}", b'A');
    let x = '中';
    println!("字符'中'占用了{}字节的内存大小",std::mem::size_of_val(&x));

    let mut v = vec![1,2,3,4,5];
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
}
