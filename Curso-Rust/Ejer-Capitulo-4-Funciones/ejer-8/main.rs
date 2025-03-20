/*      Ejer    8   Escribir una función ( fun ) a la que se le pase un número entero y devuelva el número de
                    divisores primos que tiene.

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

            println!("\tDivisores Primos de [ {} ]:\t{}\n", num, get_num_divisores_primos(num));
    
        }
}

/*  FUNCIÓN QUE DEVUELVE [ TRUE || FALSE ] SI EL NÚM INTRODUCIDO POR PARÁMETRO ES PRIMO     */
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

/*  FUNCIÓN QUE DEVUELVE CANTIDAD DE DIVISORES PRIMOS [ I32 ] DE UN NÚM INTRODUCIDO POR PARÁMETRO     */
fn get_num_divisores_primos(num: i32) -> i32 {
    let mut result : i32 = 1;

        for i in 2..num + 1 {

            if is_primo(i) && num % i == 0  {
                result += 1;
            }
        } 

    return result;
}
