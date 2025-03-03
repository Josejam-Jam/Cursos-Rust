/*      Ejer  2     Escribir una función a la que se le pasen dos enteros y muestre todos los números
                    comprendidos entre ellos.

*/

use std::io;
use std::str::FromStr;
use regex::Regex;

fn main() {
    let mut num_1: i32;
    let mut num_2: i32;
    let mut rang_nums: Vec<i32> = Vec::new();

    let exp_regex = Regex::new(r"\d+").unwrap();
    let exp_regex_caracter = Regex::new(r"[a-zA-Z]+").unwrap();
    let exp_regex_white_space = Regex::new(r"\s+").unwrap();

    let mut data_ok: bool;
    let mut data_ok_all: Vec<bool> = Vec::new();

    loop {
        println!("\nIntroduce el Número 1 del Rango [ N_1 - m_2 ]");
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

                    data_ok = exp_regex.is_match(&num_1.to_string()) && (num_1 >= 0 || num_1 <= 0);

                    if data_ok {
                        data_ok_all.push(true);
                        rang_nums.push(num_1);

                        break;
                    }
                }
            }
    }

    loop {
        println!("\nIntroduce el Número 2 del Rango [ n_1 - M_2 ]");
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

                    data_ok = exp_regex.is_match(&num_2.to_string()) && (num_2 >= 0 || num_2 <= 0);

                    if data_ok {
                        data_ok_all.push(true);
                        rang_nums.push(num_2);

                        break;
                    }
                }
            }
    }

    if data_ok_all.len() == 2 {
        data_ok = is_all_data_ok(data_ok_all);

            if data_ok {
                /*  let mut data_rang : Vec<i32> = Vec::new();
                    data_rang = rang_nums;
                    let data_rang = rang_nums.clone();
                */

                data_ok = is_iteration_positive(rang_nums.clone());

                    if data_ok {
                        print_iteration_positive(rang_nums.clone());
                    }

                data_ok = is_iteration_negative(rang_nums.clone());

                    if data_ok {
                        print_iteration_negative(rang_nums.clone());
                    }

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

/*
fn are_positives(data_rang: Vec<i32>) -> bool {
    let aux_rang: Vec<i32> = data_rang.clone();

    return aux_rang[0] >= 0 && aux_rang[1] >= 0;
}

fn are_negatives(data_rang: Vec<i32>) -> bool {
    let aux_rang: Vec<i32> = data_rang.clone();

    return aux_rang[0] < 0 && aux_rang[1] < 0;
}
*/

fn are_positives(num_1: i32, num_2: i32) -> bool {
    return num_1 >= 0 && num_2 >= 0;
}

fn are_negatives(num_1: i32, num_2: i32) -> bool {
    return num_1 < 0 && num_2 < 0;
}

/*  fn is_mayor(num_1: i32, num_2: i32) -> bool {
    return num 1 > num_2;
}
*/

/*  fn is_negative(num: i32) -> bool {
    return num < 0;
}
*/

fn is_iteration_positive(rang_data: Vec<i32>) -> bool {
    let mut result = false;
    /*  [ OPTIONS 2 ] */
    /*  let aux_rang = rang_data.clone();
        let aux_rang = rang_data;
    */
        /*  [ OPTIONS 2 ] */
        /*  if (are_positives(aux_rang[0], aux_rang[1]) || are_negatives(aux_rang[0], aux_rang[1]) || aux_rang[0] < aux_rang[1]) && aux_rang[0] <= aux_rang[1] {
                result = true;
            }
        */

        if (are_positives(rang_data[0], rang_data[1]) || are_negatives(rang_data[0], rang_data[1]) || rang_data[0] < rang_data[1]) && rang_data[0] <= rang_data[1] {
            result = true;
        }

    return result;
}

fn is_iteration_negative(rang_data: Vec<i32>) -> bool {
    let mut result = false;
    /*  [ OPTIONS 2 ] */
    /*  let aux_rang = rang_data.clone();
        let aux_rang = rang_data;
    */

        /*  [ OPTIONS 2 ] */
        /*  if (are_positives(aux_rang[0], aux_rang[1]) || are_negatives(aux_rang[0], aux_rang[1]) || aux_rang[0] > aux_rang[1]) && aux_rang[0] >= aux_rang[1] {
                result = true;
            }
        */

        if (are_positives(rang_data[0], rang_data[1]) || are_negatives(rang_data[0], rang_data[1]) || rang_data[0] > rang_data[1]) && rang_data[0] >= rang_data[1] {
            result = true;
            println!("\tEntra es_iteración negativa\n");
        }


    return result;
}

fn print_iteration_positive(rang_data: Vec<i32>) {
    
    for i in rang_data[0]..rang_data[1] {

        if i > rang_data[0] && i < rang_data[1] {
            println!("\t{}\n", i);
            // print!("\t{}", i);
        }
    }
}

fn print_iteration_negative(rang_data: Vec<i32>) {

    for i in -rang_data[0]..= -rang_data[1] {
        
        if -i < rang_data[0] && -i > rang_data[1] {
            println!("\t{}\n", -i);
            // print!("\t{}", i);
        }
    }
}
