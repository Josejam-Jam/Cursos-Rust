/*      Ejer    7   Diseñar una función que nos diga si un número es primo.

*/

use std::io;
use std::str::FromStr;
use regex::Regex;

fn main() {
    let mut num : i32;

    let exp_regex = Regex::new(r"\d+").unwrap();
    let exp_regex_caracter = Regex::new(r"[a-zA-Z]+").unwrap();
    let exp_regex_white_space = Regex::new(r"\s+").unwrap();

    let mut data_ok: bool;
 
        loop {
            println!("\nIntroduce un Número [ Entero ]");
            let mut entrada = String::new();

            io::stdin().read_line(&mut entrada).expect("Error en la lectura de datos");

            let aux_ok = &entrada.is_empty();

                if !*aux_ok {
                    let aux_data_ok: bool = exp_regex_caracter.is_match(&entrada.trim().to_string()) 
                            || exp_regex_white_space.is_match(&entrada.to_string()) && !exp_regex.is_match(&entrada.trim().to_string());

                    if aux_data_ok {
                        println!("\nError al introducir los Datos");
                    }

                    if exp_regex.is_match(&entrada.trim().to_string()) {
                        num = i32::from_str(&entrada.trim()).unwrap();

                        data_ok = exp_regex.is_match(&num.to_string());

                            if data_ok {
                                break;
                            }
                    }
                }
        }
        
        if data_ok {

            // let es_primo : bool = is_primo(num);

            // println!("\tEl Número {}\t es:\t{}\n", num, es_primo);
            println!("\tEl Número {}\t es Primo:\t{}\n", num, is_primo(num));

            /*
                for i in 1..num + 1 {
                    println!("\tEl Número {}\t es Primo:\t{}\n", i, is_primo(i));
                }
            */      
        }
}

fn is_primo(num: i32) -> bool {
    let mut result : bool = true;
    let mut init: i32 = 2;

        while init <= ((num as f32).sqrt() as i32) && result == true  {
            if num % init == 0 {
                result = false;
            }

            init += 1;
        }

    return result;
}

