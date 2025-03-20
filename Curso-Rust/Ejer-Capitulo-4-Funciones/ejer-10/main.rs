/*      Ejer    10      Escribir una función ( fun ) que decida si dos números enteros positivos son amigos.

                        Dos números son amigos si la suma de sus divisores propios ( distintos de ellos mismos )
                        son iguales entres si.

                        Es decir, sum_divisores_propios de a = b y viceversa, sum_divisores_propios de b = a

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
            println!("\nIntroduce un Número [ Entero + ]");
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
            println!("\nIntroduce un Número [ Entero + ]");
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

                    let result : bool = are_friends(num_1, num_2);
                    println!("\nLos Números [ {}, {} ]\t¿Son Números Amigo?\t{}\n", num_1, num_2, result);
                    
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

/*  FUNCIÓN DEVUELVE UN VECTOR<I32> CON LOS DIVISORES PROPIOS DE UN NÚM INTRODUCIDO POR PARÁMETRO   */
fn get_divisores_propios(num: i32) -> Vec<i32> {
    let mut result : Vec<i32> =  Vec::new();

        for i in 1..num {
            let aux: i32 = i as i32;

                if num % aux == 0 && num != aux {
                    result.push(aux);
                }
        }

    return result;
}

/*  FUNCIÓN DEVUELVE LA SUMA DE LOS DIVISORES PROPIOS DE UN NÚM:I32 INTRODUCIDO POR PARÁMETRO   */
/*  fn get_sum_divisores(num: i32) -> i32 {
    let mut result : i32 = 0;
    let aux_data : Vec<i32> = get_divisores_propios(num);

        for value in aux_data.iter() {
            result += *value;
        }

    return result;
}
*/

/*  FUNCIÓN DEVUELVE LA SUMA DE LOS DIVISORES PROPIOS DE UN VECTOR<I32> INTRODUCIDO POR PARÁMETRO   */
fn get_sum_divisores_vector(data_divisores: Vec<i32>) -> i32 {
    let mut result : i32 = 0;

        for value in data_divisores.iter() {
            result += *value;
        }

    return result;
}

/*  FUNCIÓN DEVUELVE UN BOOLEANO [ TRUE || FALSE ] SI SON NÚMEROS AMIGOS LOS NÚMEROS INTRODUCIDOS POR PARÁMETROS  */
fn are_friends(num_1: i32, num_2: i32) -> bool {
    let mut result : bool = false;

    let vec_divsores_num_1 : Vec<i32> = get_divisores_propios(num_1); 
    let vec_divisores_num_2 : Vec<i32> = get_divisores_propios(num_2);

    let sum_div_num_1 : i32 = get_sum_divisores_vector(vec_divsores_num_1);
    let sum_div_num_2 : i32 = get_sum_divisores_vector(vec_divisores_num_2);

        if sum_div_num_1 == num_2 && sum_div_num_2 == num_1 {
            result = true;
        }


    return result;
}

