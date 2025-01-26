/*      Ejer    8   Pedir un número y calcular su factorial. 
                    Por ejemplo, el factorial de 5 se denota 5! y es igual a
                    5 * 4 * 3 * 2 * 1 = 120.       
*/

use std::io;
use std::str::FromStr;
use regex::Regex;

fn main() {
    let mut num: i32;

    let exp_regex = Regex::new(r"\d+").unwrap();
    let exp_regex_caracter = Regex::new(r"[a-zA-Z]+").unwrap();
    let exp_regex_white_space = Regex::new(r"\s+").unwrap();

    let mut data_ok: bool;

        loop {
            println!("\nIntroduce un Número [ ENTERO ] para calcular el Factorial [ !n ]");
            let mut entrada_1 = String::new();

            io::stdin().read_line(&mut entrada_1).expect("Error en la lectura de datos");

            let aux_ok:&bool = &entrada_1.is_empty();

                if !*aux_ok {

                    let aux_data_ok:bool = exp_regex_caracter.is_match(&entrada_1.trim().to_string()) || exp_regex_white_space.is_match(&entrada_1.to_string()) && !exp_regex.is_match(&entrada_1.trim().to_string());
                    // let aux_data_ok:bool = exp_regex_caracter.is_match(&entrada_1.trim().to_string());

                    // if exp_regex_caracter.is_match(&entrada_1.trim().to_string()) {
                        if aux_data_ok {
                            println!("\nError al introducir los Datos Introduzca un número");
                        } 
                        
                        if exp_regex.is_match(&entrada_1.trim().to_string())  {
        
                            num = i32::from_str(&entrada_1.trim()).unwrap();
        
                            data_ok = exp_regex.is_match(&num.to_string());
        
                                // if exp_regex.is_match(&num.to_string()) {
                                if data_ok {

                                    let factorial = get_calc_factorial(num);

                                    println!("\n5!  ->\t{}\n", factorial);

                                    break;
                                    
                                } else {
                                    println!("\nError al introducir los Datos");
                                }
                        }

                }
        }
}

fn get_calc_factorial(num: i32) -> i32 {
    let mut result: i32 = num;
    let mut aux_index: i32 = num;

        loop {

            if aux_index == 1 {
                break;
            } else {

                result = result * (aux_index - 1);
                aux_index -= 1; 
                
            }  
        }

    return result;
}
