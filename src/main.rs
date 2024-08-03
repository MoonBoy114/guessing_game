use std::{cmp::Ordering, io};

use rand::Rng;


fn main() {
    println!("Угадайте число!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Секретное число равно {}", secret_number);
    loop {
    println!("Введите свою догадку!");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Не удалось определить число!");
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {println!("Неверный тип данных, попробуйте еще раз");
    continue;},
    };
    println!("Ваше загаданное число: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Слишком малое число!"),
        Ordering::Equal =>{ println!("Вы угадали!");
        break;
    },
        Ordering::Greater => println!("Слишком большое число!"),
    }
    }


    


}
