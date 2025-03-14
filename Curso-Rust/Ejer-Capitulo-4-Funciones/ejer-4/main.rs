/*      Ejer    4   Diseñar una función que reciba como parámetros dos números enteros y que devuelva el
                    máximo ( el mayor de los dos ).

*/

use std::io;
use std::str::FromStr;
use regex::Regex;

fn main() {
    let mut num_1 : i32;
    let mut num_2 : i32;

    let exp_regex = Regex::new(r"\d+").unwrap();
    let exp_regex_caracter = Regex::new(r"[a-zA-Z]+").unwrap();
    let exp_regex_white_space = Regex::new(r"\s+").unwrap();

    let mut data_ok: bool;
    let mut data_ok_all: Vec<bool> = Vec::new();
 
        loop {
            println!("\nIntroduce un Número [ Entero ]");
            let mut entrada = String::new();

            io::stdin().read_line(&mut entrada).expect("Error en la lectura de datos");

            let aux_ok = &entrada.is_empty();

                if !*aux_ok {
                    let aux_data_ok: bool = exp_regex_caracter.is_match(&entrada.trim().to_string()) || exp_regex_white_space.is_match(&entrada.to_string()) && !exp_regex.is_match(&entrada.trim().to_string());

                    if aux_data_ok {
                        println!("\nError al introducir los Datos");
                    }

                    if exp_regex.is_match(&entrada.trim().to_string()) {
                        num_1 = i32::from_str(&entrada.trim()).unwrap();

                        data_ok = exp_regex.is_match(&num_1.to_string());

                            if data_ok {
                                data_ok_all.push(true);

                                break;
                            }
                    }
                }
        }

        loop {
            println!("\nIntroduce un Número [ Entero ]");
            let mut entrada = String::new();

            io::stdin().read_line(&mut entrada).expect("Error en la lectura de datos");

            let aux_ok = &entrada.is_empty();

                if !*aux_ok {
                    let aux_data_ok: bool = exp_regex_caracter.is_match(&entrada.trim().to_string()) || exp_regex_white_space.is_match(&entrada.to_string()) && !exp_regex.is_match(&entrada.trim().to_string());

                    if aux_data_ok {
                        println!("\nError al introducir los Datos");
                    }

                    if exp_regex.is_match(&entrada.trim().to_string()) {
                        num_2 = i32::from_str(&entrada.trim()).unwrap();

                        data_ok = exp_regex.is_match(&num_2.to_string());

                            if data_ok {
                                data_ok_all.push(true);

                                break;
                            }
                    }
                }
        }

        
        if data_ok_all.len() == 2 {
            data_ok = is_all_data_ok(data_ok_all);

                if data_ok {
                    let num_mayor : i32 = get_num_mayor(num_1, num_2);

                    println!("\tEl Número Mayor [ {} - {} ]\t es:\t{}\n", num_1, num_2, num_mayor);
                    
                }
        }
}

/*  REALIZAR EL MATCH  IF DATA-IN == FALSE */

fn is_all_data_ok(data_ok_all: Vec<bool>) -> bool {
    let mut result: bool = false;

        for value in data_ok_all.iter() {
            if *value == false {
                return false;
            } else {
                result = true;
            }
        }

    return result;
}

fn is_mayor(num_1: i32, num_2: i32) -> bool {
    return num_1 > num_2;
}

fn get_num_mayor(num_1: i32, num_2: i32) -> i32 {
    // let is_mayor: bool = is_mayor(num_1, num_2);
    let result: i32;
        
        // if is_mayor {
        if is_mayor(num_1, num_2) {
            result = num_1;
        } else {
            result = num_2;
        } 

    return result;
}
