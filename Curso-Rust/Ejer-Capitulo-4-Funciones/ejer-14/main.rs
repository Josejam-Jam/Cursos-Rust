/*      Ejer 13         Diseñar una función que calcule el nésimo término de la serie de Fibonacci. En esta serie el
                        nénesimo valor se calcula sumando los dos valores anteriores. Es decir:


                        fibonacci ( n ) = fibonacci ( n -1 ) + fibonacci ( n - 2 )

                        fibonacci ( 0)

                        fibonacci ( 1 ) ej 
*/

            /*  ***     TO DO       
            */

use std::io;
use std::str::FromStr;
use regex::Regex;


fn main() {
    let mut num : i32;

    let exp_regex = Regex::new(r"\d+").unwrap();
    let exp_regex_caracter = Regex::new(r"[a-zA-Z]+").unwrap();
    let exp_regex_white_space = Regex::new(r"\s+").unwrap();

    let exp_regex_num_negative = Regex::new(r"-\d+").unwrap();

    /*  *** REGEX PARA LA VERSIÓN DE TIPOS FLOAT      */
    /*let exp_regex_data_incomplete = Regex::new(r"(\d+)[\,||\.||\']").unwrap();

    let exp_regex_num_float = Regex::new(r"\d+\.\d+").unwrap();
    */

    let mut data_ok: bool;
    let mut data_ok_all: Vec<bool> = Vec::new();

        loop {
            println!("\nIntroduce un Número");
            let mut entrada = String::new();

            io::stdin().read_line(&mut entrada).expect("Error en la lectura de datos");

            let aux_ok = &entrada.is_empty();

                if !*aux_ok {
                    let aux_data_ok: bool = exp_regex_caracter.is_match(&entrada.trim().to_string()) 
                            || exp_regex_white_space.is_match(&entrada.to_string()) 
                            && !exp_regex.is_match(&entrada.trim().to_string());

                    let aux_data_negative: bool = exp_regex_num_negative.is_match(&entrada.to_string());

                        if aux_data_ok || aux_data_negative {
                            println!("\nError al introducir los Datos");
                        }

                        /*  *** REGEX PARA LA VERSIÓN DE TIPOS FLOAT      */
                        //let aux_data_incomplete: bool = exp_regex_data_incomplete.is_match(&entrada.to_string()) && !exp_regex_num_float.is_match(&entrada.to_string());

                        // if aux_data_incomplete {
                        //     println!("\nError al introducir los Datos 2");
                        // }

                        if !aux_data_ok {

                            if exp_regex.is_match(&entrada.trim().to_string()) && !exp_regex_num_negative.is_match(&entrada.to_string()) {
                                num = i32::from_str(&entrada.trim()).unwrap();

                                data_ok = exp_regex.is_match(&num.to_string());
            
                                    if data_ok{

                                        data_ok_all.push(true);

                                        break;
                                    }
                            }
                        }
                    
                }
        }
        
        if data_ok_all.len() == 1 {
            data_ok = is_all_data_ok(data_ok_all);

                if data_ok {

                    print!("\nfibonacci( {} ) = ", num);
                    print!("{}\n\n", get_secuence_fibonacci(num));

                }
        }

}

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


fn get_secuence_fibonacci(num: i32) -> i32 {
    let result: i32;

        if num == 0 {
            result = 1;

        } else {

            if num == 1 {
                result = 1;
            } else {
                result = get_secuence_fibonacci(num - 1) + get_secuence_fibonacci(num - 2);
            }

        }

    return result;
}
