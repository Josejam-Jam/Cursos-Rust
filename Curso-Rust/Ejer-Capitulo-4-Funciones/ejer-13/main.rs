/*      Ejer 13         Calcular el factorial de n recursivamente. Recordar que por definición el factorial de 0 es
                        ( 0! ) es 1.  
*/

            /*  ***     TO DO   FUTURIBLE REALIZAR UNA VERSIÓN PARA EL CÁLCULO DEL FACTORIAL DE UN NÚMERO NEGATIVO

                ***     TO DO   REALIZAR UNA VERSIÓN PARA LA RECEPCIÓN DE PARÁMETROS TIPO FLOAT,
                                TANTO PARA LA ENTRADA DE DATOS, ASÍ COMO PARA LA PARAMETRIZACIÓN DE LAS FUNCIONES       
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
                            && !exp_regex.is_match(&entrada.trim().to_string()) 
                            && exp_regex_num_negative.is_match(&entrada.to_string());

                    if aux_data_ok  {
                        println!("\nError al introducir los Datos 1");
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

                    print!("\n{}! = ", num);
                    print!("{}\n\n", get_calc_factorial_recursive(num));

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

fn get_calc_factorial_recursive(num: i32) -> i32 {
    
    if num == 0 { 
        return 1;  
                
    } else {
               
        print!("{} ", num);
        return num * get_calc_factorial_recursive(num - 1);
                
    } 
}

/*  *** FN CÁLCULA && SHOW FACTORIAL DE UN NÚMERO NEGATIVO       */
fn get_calc_factorial_num_negative_recursive(num: i32) -> i32 {
    if num == 0 { 
        return 1;  
                
    } else {
        print!("{} ", num);
        return num * get_calc_factorial_num_negative_recursive(num + 1);
                
    }  
}

