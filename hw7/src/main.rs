use library::{create, read, write};

fn main() {
    let mut buffer = create(3);
    let step1 = write(&mut buffer, vec!['a', 'b']);
    println!("{:?}", step1);
    let step2 = write(&mut buffer, vec!['c', 'd']);
    println!("{:?}", step2);
    let step3 = read(&mut buffer, 1);
    println!("{:?}", step3);
    let step4 = write(&mut buffer, vec!['e']);
    println!("{:?}", step4);
    let step5 = read(&mut buffer, 2);
    println!("{:?}", step5);
}
