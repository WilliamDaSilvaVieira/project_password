use std::io;

use rand::Rng;

fn main() {
    println!("Bem Vindo ao gerador de senhas!");
    let tamanho: u8 = loop {
        println!("Quantos caracteres quer em sua senha? (De 8 à 16)");
        let mut numero = String::new();
        io::stdin()
            .read_line(&mut numero)
            .expect("Falha ao obter o número.");
        match numero.trim().parse() {
            Ok(num) => match num {
                8..=16 => break num,
                _ => {
                    println!("Digite um número de 8 à 16!");
                    continue;
                }
            },
            Err(_) => {
                println!("Digite apenas números! Somente números positivos!");
                continue;
            }
        };
    };
    println!("Sua senha gerada é:\n\n{}\n\n", password(tamanho));
}

fn password(t: u8) -> String {
    let mut verificador = vec![0, 0, 0, 0];
    let numbers = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let lower = vec![
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z",
    ];
    let upper = vec![
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R",
        "S", "T", "U", "V", "W", "X", "Y", "Z",
    ];
    let specials = vec![
        "!", "@", "'", "#", "%", "&", "*", "(", ")", "-", "_", "=", "+", "[", "]", "{", "}", "?",
        ":", ";", ">", ".", "<", ",",
    ];
    let mut password = String::new();
    let mut y: u8;
    let mut index: usize;
    for x in 1..=t {
        y = rand::thread_rng().gen_range(1..=4);
        if x >= (t / 2) {
            if verificador[0] == 0 {
                index = rand::thread_rng().gen_range(0..10);
                password.push_str(&numbers[index]);
                verificador[0] = 1;
                continue;
            }
            if verificador[1] == 0 {
                index = rand::thread_rng().gen_range(0..26);
                password.push_str(&lower[index]);
                verificador[1] = 1;
                continue;
            }
            if verificador[2] == 0 {
                index = rand::thread_rng().gen_range(0..26);
                password.push_str(&upper[index]);
                verificador[2] = 1;
                continue;
            }
            if verificador[3] == 0 {
                index = rand::thread_rng().gen_range(0..24);
                password.push_str(&specials[index]);
                verificador[3] = 1;
                continue;
            } else {
                match y {
                    1 => {
                        index = rand::thread_rng().gen_range(0..10);
                        password.push_str(&numbers[index]);
                    }
                    2 => {
                        index = rand::thread_rng().gen_range(0..26);
                        password.push_str(&lower[index]);
                    }
                    3 => {
                        index = rand::thread_rng().gen_range(0..26);
                        password.push_str(&upper[index]);
                    }
                    _ => {
                        index = rand::thread_rng().gen_range(0..24);
                        password.push_str(&specials[index]);
                    }
                }
            }
        } else {
            match y {
                1 => {
                    index = rand::thread_rng().gen_range(0..10);
                    password.push_str(&numbers[index]);
                    verificador[0] = 1;
                }
                2 => {
                    index = rand::thread_rng().gen_range(0..26);
                    password.push_str(&lower[index]);
                    verificador[1] = 1;
                }
                3 => {
                    index = rand::thread_rng().gen_range(0..26);
                    password.push_str(&upper[index]);
                    verificador[2] = 1;
                }
                _ => {
                    index = rand::thread_rng().gen_range(0..24);
                    password.push_str(&specials[index]);
                    verificador[3] = 1;
                }
            }
        }
    }
    password
}
