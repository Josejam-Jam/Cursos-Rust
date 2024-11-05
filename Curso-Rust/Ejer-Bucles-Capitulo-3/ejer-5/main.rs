/*      Ejer    5       Desarrollar un programa que solicite los valores mínimo y máximo de un rango.

                        A continuación solicitará por teclado un número que debe estar dentro del rango. 
                        
                        Si el valor introducido no pertenece al rango, la aplicación volverá a pedir otro valor,
                        y así repetidas veces, hasta que el valor se encuentre dentro del rango.

*/

use std::io;
use std::str::FromStr;
use regex::Regex;

fn main() {

    let mut num_min: i32;
    let mut num_max: i32;
    let mut num_user: i32;
    let exp_regex = Regex::new(r"\d+").unwrap();
    let exp_regex_caracter = Regex::new(r"[a-zA-Z]+").unwrap();
    let exp_regex_white_space = Regex::new(r"\s+").unwrap();

    let mut data_ok: bool;

        loop {
            println!("\nIntroduce un Número para el Rango Mínimo del Intervalo");
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
    
                        num_min = i32::from_str(&entrada_1.trim()).unwrap();
    
                        data_ok = exp_regex.is_match(&num_min.to_string());
    
                            // if exp_regex.is_match(&num_min.to_string()) {
                            if data_ok {
                                break;
                            } else {
                                println!("\nError al introducir los Datos");
                            }
                    }

            }

        }

        loop {
            println!("\nIntroduce un Número para el Rango Máximo del Intervalo");
            let mut entrada_2 = String::new();

            io::stdin().read_line(&mut entrada_2).expect("Error en la lectura de datos");

            let aux_ok:&bool = &entrada_2.is_empty();

                if !*aux_ok {
                    let aux_data_ok:bool = exp_regex_caracter.is_match(&entrada_2.trim().to_string()) || exp_regex_white_space.is_match(&entrada_2.to_string()) && !exp_regex.is_match(&entrada_2.trim().to_string());

                    // if exp_regex_caracter.is_match(&entrada_2.trim().to_string()) {
                    if aux_data_ok {
                        println!("\nError al introducir los Datos Introduzca un número");
                        
                    } 
                    
                    if exp_regex.is_match(&entrada_2.trim().to_string())  {

                        num_max = i32::from_str(&entrada_2.trim()).unwrap();

                        data_ok = exp_regex.is_match(&num_max.to_string());

                            // if exp_regex.is_match(&num_max.to_string()) {
                            if data_ok && num_max > num_min {
                                break;
                            } else {
                                println!("\nError al introducir los Datos");
                            }

                    }

                }    
        }

        loop {
            println!("\nIntroduce un Número que pertenezca al intervalo [ {} - {} ]", num_min, num_max);
            let mut entrada_3 = String::new();

            io::stdin().read_line(&mut entrada_3).expect("Error en la lectura de datos");

            let aux_ok:&bool = &entrada_3.is_empty();

                if !*aux_ok {

                    let aux_data_ok:bool = exp_regex_caracter.is_match(&entrada_3.trim().to_string()) || exp_regex_white_space.is_match(&entrada_3.to_string()) && !exp_regex.is_match(&entrada_3.trim().to_string());

                    // if exp_regex_caracter.is_match(&entrada_3.trim().to_string()) {
                    if aux_data_ok {
                        println!("\nError al introducir los Datos Introduzca un número");
                        
                    }
                    
                    if exp_regex.is_match(&entrada_3.trim().to_string())  { 

                        num_user = i32::from_str(&entrada_3.trim()).unwrap();

                        data_ok = exp_regex.is_match(&num_user.to_string());

                            // if exp_regex.is_match(&num_user.to_string()) && is_range(num_min, num_max, num_user) {
                            if data_ok && is_range(num_min, num_max, num_user) {
                                println!("\nEl Número Introducido\t{}\testá dentro del Rango\t[ {} - {} ]\n", num_user, num_min, num_max);
                                break;
                            } else {
                                println!("\nError al introducir los Datos");
                            }

                    }

                }    
    
        }

}

fn is_range(num_min: i32, num_max: i32, num: i32) -> bool {
    let mut result: bool = false;

        if (num >= num_min) && (num <= num_max) { 
            result = true; 
        }

    return result;

    // return (num >= num_min) && (num <= num_max);
}
