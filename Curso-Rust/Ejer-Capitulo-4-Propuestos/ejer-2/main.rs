/*      Ejer 2          Crear la función muestra_Pares( num: i32 ) que muestre por consola los primeros n números
                        pares.

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
            
    // let exp_regex_num_negative = Regex::new(r"(-\d+)||(-\d+\.\d+)").unwrap();
            
    /*  *** REGEX PARA LA VERSIÓN DE TIPOS FLOAT      */
    //let exp_regex_data_incomplete = Regex::new(r"(\d+)[\,||\.||\']").unwrap();
            
    // let exp_regex_num_float = Regex::new(r"\d+\.\d+").unwrap();
    // let exp_regex_num_float_negative = Regex::new(r"-\d+\.\d+").unwrap();
            
    let mut data_ok: bool;
    let mut data_ok_all: Vec<bool> = Vec::new();


        loop {
            println!("\nIntroduce el Número [ Entero ] de Pares a mostrar");
            let mut entrada = String::new();

            io::stdin().read_line(&mut entrada).expect("Error en la lectura de datos");

            let aux_ok = &entrada.is_empty();

                if !*aux_ok {
                    let aux_data_ok: bool = exp_regex_caracter.is_match(&entrada.trim().to_string()) 
                            || exp_regex_white_space.is_match(&entrada.to_string()) 
                            && !exp_regex.is_match(&entrada.trim().to_string());

                    //let aux_data_negative: bool = exp_regex_num_negative.is_match(&entrada.to_string());

                    /*  *** REGEX PARA LA VERSIÓN DE TIPOS FLOAT      */
                    /*let aux_data_incomplete: bool = exp_regex_data_incomplete.is_match(&entrada.to_string()) 
                            && !exp_regex_num_float.is_match(&entrada.to_string());

                        if aux_data_ok || aux_data_negative || aux_data_incomplete {
                            println!("\nError al introducir los Datos");
                        }
                    
                        if !aux_data_ok || ( !aux_data_negative && !aux_data_incomplete ) {
                    */        
                        if !aux_data_ok {

                            if exp_regex.is_match(&entrada.to_string()) {

                                num = i32::from_str(&entrada.trim()).unwrap();

                                data_ok = exp_regex.is_match(&num.to_string());
            
                                    if data_ok && num != 0 {

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
                    let result_vector: Vec<i32> = get_n_num_pares(num);

                    show_num_pares_vector(result_vector);

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

fn is_par(num: i32) -> bool {
    return num % 2 == 0;
}

fn get_n_num_pares(num: i32) -> Vec<i32> {
    let mut result_vector: Vec<i32> = Vec::new();
    //let mut result_vector: Vec<i32> = !vec[];

    let mut aux_index: i32 = num;
    let mut aux_num: i32 = 1;

        loop {

            if is_par(aux_num) {

                result_vector.push(aux_num);
                aux_index -= 1;
            }

            if aux_index == 0 {
                break;
            }

            aux_num += 1;
        }

    return result_vector;
}

fn show_num_pares_vector(num_vector: Vec<i32>) {
    let tam: usize = num_vector.len();

    println!("\n{}  Próximos Números Pares\t{:?}\n", tam, num_vector);
}
