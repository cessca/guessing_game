use core::num;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("数を予想してください");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("秘密の数字は{}", secret_number);

    loop {
        println!("予想を入力してください");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("行の読み込みに失敗しました");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("あなたの予想は:{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小さすぎるよ"),
            Ordering::Greater => println!("大きすぎるよ"),
            Ordering::Equal => {
                println!("正解！！");
                break;
            }
        }
    }
}
