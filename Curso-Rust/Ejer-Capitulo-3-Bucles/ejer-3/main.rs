/*  Ejer 3      Realizar el juego, “EL NÚMERO SECRETO”, que consiste en acertar un número
                desconocido (generado aleatoriamente entre 1 y 100). 
                
                Para ello se leen por teclado una serie de números, para los que se indica: <<mayor>> o <<menor>>,
                según se mayor o menor con respeto al “Número Secreto”.
                
                El proceso termina cuando el usuario acierta o cuando se rinde (introduciendo un -1).
                    
*/

use std::io;
use std::str::FromStr;
use regex::Regex;
use rand::Rng;


fn main() {

    let num_secret: i32;
    const NUM_1_RANGE: u32 = 1;
    const NUM_2_RANGE: u32 = 101;
    //let exp_regex = Regex::new(r"[\d+][1-3]").unwrap();
    let exp_regex = Regex::new(r"\d+").unwrap();

    //let mut num_secret: i32 = rand::thread_rng().gen_range(1..101);
    //num_secret = get_num_aleatorio_rand(1, 101);
    num_secret = get_num_aleatorio_rand(NUM_1_RANGE, NUM_2_RANGE);

        loop {
            println!("\nIntroduce un Número entre [ 1 - 100 ]");
            let mut entrada = String::new();

            io::stdin().read_line(&mut entrada).expect("Error en la lectura de datos");

            let num = i32::from_str(&entrada.trim()).unwrap();

                if exp_regex.is_match(&num.to_string()) {

                    if num == -1 { println!("\nPor Fin Te has Rendido..!!\n"); break; }

                    if num == num_secret {
                        println!("\nHas Aceertado el número secreto:\t {}\n", num_secret);
                        break;
                    } else {

                        if num < num_secret {
                            println!("\nEl Número es Mayor");
                        } else {
                            println!("\nEl Número es Menor");
                        }

                        println!("\nTe puedes Rendir si quieres... pulsa [ - 1 ]");
                    }

                } else {
                    println!("\nError al introducir los Datos\n");
                }

        }

}

fn get_num_aleatorio_rand(num_1: u32, num_2: u32) -> i32 {
    return (rand::thread_rng().gen_range(num_1..num_2)) as i32;
}

/*
fn get_num_secret(num_1: u32, num_2: u32) -> i32 {
    return get_num_aleatorio_rand(num_1, num_2);
}
*/

