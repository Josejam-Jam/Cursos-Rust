/*      Ejer    4   Diseñar una función que reciba como parámetros dos números enteros y que devuelva el
                    máximo ( el mayor de los dos ).

        This -> Ejer    5    Repetir el ejercicio anterior con una versión que calcule el máximo de 3 números.

*/

use std::io;
use std::str::FromStr;
use regex::Regex;

fn main() {
    let mut num_1 : i32;
    let mut num_2 : i32;
    let mut num_3 : i32;
    let mut data_nums: Vec<i32> = Vec::new();

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
                                data_nums.push(num_1);

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
                                data_nums.push(num_2);

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
                    let aux_data_ok: bool = exp_regex_caracter.is_match(&entrada.trim().to_string()) 
                            || exp_regex_white_space.is_match(&entrada.to_string()) && !exp_regex.is_match(&entrada.trim().to_string());

                    if aux_data_ok {
                        println!("\nError al introducir los Datos");
                    }

                    if exp_regex.is_match(&entrada.trim().to_string()) {
                        num_3 = i32::from_str(&entrada.trim()).unwrap();

                        data_ok = exp_regex.is_match(&num_3.to_string());

                            if data_ok {
                                data_ok_all.push(true);
                                data_nums.push(num_3);

                                break;
                            }
                    }
                }
        }

        
        if data_ok_all.len() == 3 {
            data_ok = is_all_data_ok(data_ok_all);

                if data_ok {

                    if data_nums.len() == 3 {
                        /*  *** VERSIÓN QUE RECIBE 3 PARÁMETROS INDEPENDIENTES TIPO : I32       */
                        //let num_mayor : i32 = get_num_mayor(data_nums[0], data_nums[1], data_nums[2]);

                        /*  *** VERSIÓN QUE RECIBE UN VECTOR<I32>  DE ELEM TIPO : I32       */
                        let num_mayor : i32 = get_num_mayor(data_nums);

                        println!("\tEl Número Mayor [ {} , {} , {} ]\t es:\t{}\n", num_1, num_2, num_3, num_mayor);
                    
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

fn is_mayor(num_1: i32, num_2: i32) -> bool {
    return num_1 > num_2;
}

/*  *** VERSIÓN QUE RECIBE 3 PARÁMETROS INDEPENDIENTES TIPO : I32       */ 
/*fn get_num_mayor(num_1: i32, num_2: i32, num_3: i32) -> i32 {
    // let is_mayor: bool = is_mayor(num_1, num_2);
    let mut result: i32;
        
        // if is_mayor {
        if is_mayor(data_nums[0], data_nums[1]) {
            result = data_nums[0];
        } else {
            result = data_nums[1];
        }
    
        if !is_mayor(result, data_nums[2]) {
            result = data_nums[2];
        } 

    return result;
}
*/

/*  *** VERSIÓN QUE RECIBE UN VECTOR<I32>  DE ELEM TIPO : I32       */
fn get_num_mayor(data_nums: Vec<i32>) -> i32 {
    // let is_mayor: bool = is_mayor(data_nums[0], data_nums[1]);
    let mut result: i32 = 0;

        /*  *** OPCIÓN 1    Versión EXCLUSIVA Y ENFOCADA PARA UN VEC<I32>.LEN() == 3  */
        // if is_mayor {
        /*if is_mayor(data_nums[0], data_nums[1]) {
            result = data_nums[0];
        } else {
            result = data_nums[1];
        }

        if !is_mayor(result, data_nums[2]) {
            result = data_nums[2];
        }  
        */

        /*  *** OPCIÓN 2    Versión EXCLUSIVA Y ENFOCADA PARA UN VEC<I32>.LEN() == *??  TAM == INDETERMINADO  */ 

    let mut ind: usize = 0;
    let size_vec_data: usize = data_nums.len();

        for i in data_nums.iter() {

            // if ind == 3 {  return result; }
            // if ind + 1 < 3 {
            if ind + 1 < size_vec_data {
                let aux_data = data_nums[ind + 1 as usize];
            
                    //if *i as i32 > aux_data {
                    // if is_mayor(*i as i32, aux_data) && result < aux_data && ind as i32 >= 0 {
                    if is_mayor(*i as i32, aux_data) && !is_mayor(result, aux_data) && ind as i32 >= 0 {
                        result = *i;
                    } else {
                        if result < aux_data {
                            result = aux_data;
                        } else {
                            result = result;
                        }

                        /*if is_mayor(result, aux_data)  {
                            result = aux_data;
                        } else {
                            result = result;
                        } */ 

                    }
            
                ind += 1;

                // if ind == 3 {   break;  }
                if ind == size_vec_data {   break;  }
            }
            
        }
        
    return result;
}

