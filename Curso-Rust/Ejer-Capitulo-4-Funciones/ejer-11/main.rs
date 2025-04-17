/*      Ejer    11      Diseñar una función ( fun ) que calcule a ^ n , donde a es real y n entero no es negativo.

                        Realizar una versión iterativa y otra recursiva.

*/

use std::io;
use std::str::FromStr;
use regex::Regex;

fn main() {
    let mut num : f32;
    let mut exp : i32;

    let exp_regex = Regex::new(r"\d+").unwrap();
    let exp_regex_caracter = Regex::new(r"[a-zA-Z]+").unwrap();
    let exp_regex_white_space = Regex::new(r"\s+").unwrap();
    let exp_regex_data_incomplete = Regex::new(r"(\d+)[\,||\.||\']").unwrap();

    let exp_regex_num_float = Regex::new(r"\d+\.\d+").unwrap();
    let exp_regex_num_negative = Regex::new(r"-\d+").unwrap();

    let mut data_ok: bool;
    let mut data_ok_all: Vec<bool> = Vec::new();


        loop {
            println!("\nIntroduce un Número [ Real || Decimal-Float ]");
            let mut entrada = String::new();

            io::stdin().read_line(&mut entrada).expect("Error en la lectura de datos");

            let aux_ok = &entrada.is_empty();

                if !*aux_ok {
                    let aux_data_ok: bool = exp_regex_caracter.is_match(&entrada.trim().to_string()) 
                            || exp_regex_white_space.is_match(&entrada.to_string()) && !exp_regex.is_match(&entrada.trim().to_string());

                    if aux_data_ok  {
                        println!("\nError al introducir los Datos 1");
                    }

                    let aux_data_incomplete: bool = exp_regex_data_incomplete.is_match(&entrada.to_string()) && !exp_regex_num_float.is_match(&entrada.to_string());

                    if aux_data_incomplete {
                        println!("\nError al introducir los Datos 2");
                    }

                    if !aux_data_ok || !aux_data_incomplete {

                        if exp_regex_num_float.is_match(&entrada.to_string()) {
                            num = f32::from_str(&entrada.trim()).unwrap();
    
                            data_ok = exp_regex_num_float.is_match(&num.to_string());
        
                                if data_ok{
    
                                    data_ok_all.push(true);
    
                                    break;
                                }
                        }
                    }
                    
                }
        }


        loop {
            println!("\nIntroduce un Número como Exponente [ Entero + ]");
            let mut entrada = String::new();

            io::stdin().read_line(&mut entrada).expect("Error en la lectura de datos");

            let aux_ok = &entrada.is_empty();

                if !*aux_ok {
                    let aux_data_ok: bool = exp_regex_caracter.is_match(&entrada.trim().to_string()) 
                            || exp_regex_white_space.is_match(&entrada.to_string()) 
                            && !exp_regex.is_match(&entrada.trim().to_string()) 
                            && exp_regex_num_negative.is_match(&entrada.to_string());

                    if aux_data_ok {
                        println!("\nError al introducir los Datos");
                    }

                    if exp_regex.is_match(&entrada.trim().to_string()) && !exp_regex_num_negative.is_match(&entrada.to_string()) {
                        exp = i32::from_str(&entrada.trim()).unwrap();

                        data_ok = !exp_regex_num_negative.is_match(&entrada.to_string());

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

                    let result_potencia: f32 = get_elevar_potencia(num, exp);
                    let result_potencia_2: f32 = get_elevar_potencia_iterativa(num, exp);

                    println!("\nLos Resultado de la potencia [ {}, {} ]\tResultado\t{}\n", num, exp, result_potencia);
                    println!("\nLos Resultado de la potencia [ {}, {} ]\tResultado\t{}\n", num, exp, result_potencia_2);

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


fn get_elevar_potencia(num: f32, exp: i32) -> f32 {
    return num.powf(exp as f32);
}


fn get_elevar_potencia_iterativa(num: f32, exp: i32) -> f32 {
    let mut result: f32 = num;

        for _i in 1..exp {
            result *= num;
        }

    return result;
}

