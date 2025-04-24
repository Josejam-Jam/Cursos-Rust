/*      Ejer    3       Implementar la función ( fn ), que calcula y devuelve la distancia euclídea que separa los
                        puntos ( x1, y1 ) y ( x2, y2 ).

                            fn get_distancia ( x_1 : f32, y_1: f32, x_2: f32, y_2: f32 ) -> f32

                            Fórmula para calcular esta distancia:

                                distancia =  Raíz [ ( x1 – x2 )² + ( y1 - y2 )² ]

*/

            /*  ***     TO DO       
            */

use std::io;
use std::str::FromStr;
use regex::Regex;
                        
fn main() {
    let mut punto_x_1 : f32;
    let mut punto_y_1 : f32;
    let mut punto_x_2 : f32;
    let mut punto_y_2 : f32;
            
    let exp_regex = Regex::new(r"\d+").unwrap();
    let exp_regex_caracter = Regex::new(r"[a-zA-Z]+").unwrap();
    let exp_regex_white_space = Regex::new(r"\s+").unwrap();
            
    let exp_regex_num_negative = Regex::new(r"(-\d+)").unwrap();
            
    /*  *** REGEX PARA LA VERSIÓN DE TIPOS FLOAT      */
    let exp_regex_data_incomplete = Regex::new(r"(\d+)[\,||\.||\']").unwrap();
            
    let exp_regex_num_float = Regex::new(r"\d+\.\d+").unwrap();
    let exp_regex_num_float_negative = Regex::new(r"-\d+\.\d+").unwrap();
            
    let mut data_ok: bool;
    let mut data_ok_all: Vec<bool> = Vec::new();


        loop {
            println!("\nIntroduce X - 1 [ Float ]");
            let mut entrada = String::new();

            io::stdin().read_line(&mut entrada).expect("Error en la lectura de datos");

            let aux_ok = &entrada.is_empty();

                if !*aux_ok {
                    let aux_data_ok: bool = exp_regex_caracter.is_match(&entrada.trim().to_string()) 
                            || exp_regex_white_space.is_match(&entrada.to_string()) 
                            && !exp_regex.is_match(&entrada.trim().to_string());

                    let aux_data_negative: bool = exp_regex_num_negative.is_match(&entrada.to_string());

                    /*  *** REGEX PARA LA VERSIÓN DE TIPOS FLOAT      */
                    let aux_data_incomplete: bool = exp_regex_data_incomplete.is_match(&entrada.to_string()) 
                            && !exp_regex_num_float.is_match(&entrada.to_string());

                        if aux_data_ok || aux_data_negative || aux_data_incomplete {
                            println!("\nError al introducir los Datos");
                        }
                           
                        if !aux_data_ok || ( !aux_data_negative && !aux_data_incomplete ) {

                            if exp_regex_num_float.is_match(&entrada.to_string()) && !exp_regex_num_float_negative.is_match(&entrada.to_string()) {
                                punto_x_1 = f32::from_str(&entrada.trim()).unwrap();

                                data_ok = exp_regex_num_float.is_match(&punto_x_1.to_string()) 
                                    && !exp_regex_num_float_negative.is_match(&entrada.to_string());
            
                                    if data_ok {

                                        data_ok_all.push(true);

                                        break;
                                    }
                            }
                        }
                    
                }
        }

        loop {
            println!("\nIntroduce Y - 1 [ FLOAT ]");
            let mut entrada = String::new();

            io::stdin().read_line(&mut entrada).expect("Error en la lectura de datos");

            let aux_ok = &entrada.is_empty();

                if !*aux_ok {
                    let aux_data_ok: bool = exp_regex_caracter.is_match(&entrada.trim().to_string()) 
                            || exp_regex_white_space.is_match(&entrada.to_string()) 
                            && !exp_regex.is_match(&entrada.trim().to_string());

                    let aux_data_negative: bool = exp_regex_num_negative.is_match(&entrada.to_string());

                    /*  *** REGEX PARA LA VERSIÓN DE TIPOS FLOAT      */
                    let aux_data_incomplete: bool = exp_regex_data_incomplete.is_match(&entrada.to_string()) 
                            && !exp_regex_num_float.is_match(&entrada.to_string());

                        if aux_data_ok || aux_data_negative || aux_data_incomplete {
                            println!("\nError al introducir los Datos");
                        }
                           
                        if !aux_data_ok || ( !aux_data_negative && !aux_data_incomplete ) {

                            if exp_regex_num_float.is_match(&entrada.to_string()) && !exp_regex_num_float_negative.is_match(&entrada.to_string()) {
                                punto_y_1 = f32::from_str(&entrada.trim()).unwrap();

                                data_ok = exp_regex_num_float.is_match(&punto_y_1.to_string()) 
                                    && !exp_regex_num_float_negative.is_match(&entrada.to_string());
            
                                    if data_ok {

                                        data_ok_all.push(true);

                                        break;
                                    }
                            }
                        }
                    
                }
        }

        loop {
            println!("\nIntroduce X - 2 [ FLOAT ]");
            let mut entrada = String::new();

            io::stdin().read_line(&mut entrada).expect("Error en la lectura de datos");

            let aux_ok = &entrada.is_empty();

                if !*aux_ok {
                    let aux_data_ok: bool = exp_regex_caracter.is_match(&entrada.trim().to_string()) 
                            || exp_regex_white_space.is_match(&entrada.to_string()) 
                            && !exp_regex.is_match(&entrada.trim().to_string());

                    let aux_data_negative: bool = exp_regex_num_negative.is_match(&entrada.to_string());

                    /*  *** REGEX PARA LA VERSIÓN DE TIPOS FLOAT      */
                    let aux_data_incomplete: bool = exp_regex_data_incomplete.is_match(&entrada.to_string()) 
                            && !exp_regex_num_float.is_match(&entrada.to_string());

                        if aux_data_ok || aux_data_negative || aux_data_incomplete {
                            println!("\nError al introducir los Datos");
                        }
                           
                            
                        if !aux_data_ok || ( !aux_data_negative && !aux_data_incomplete ) {

                            if exp_regex_num_float.is_match(&entrada.to_string()) && !exp_regex_num_float_negative.is_match(&entrada.to_string()) {
                                punto_x_2 = f32::from_str(&entrada.trim()).unwrap();

                                data_ok = exp_regex_num_float.is_match(&punto_x_2.to_string()) 
                                    && !exp_regex_num_float_negative.is_match(&entrada.to_string());
            
                                    if data_ok {

                                        data_ok_all.push(true);

                                        break;
                                    }
                            }
                        }
                    
                }
        }

        loop {
            println!("\nIntroduce Y - 2 [ FLOAT ]");
            let mut entrada = String::new();

            io::stdin().read_line(&mut entrada).expect("Error en la lectura de datos");

            let aux_ok = &entrada.is_empty();

                if !*aux_ok {
                    let aux_data_ok: bool = exp_regex_caracter.is_match(&entrada.trim().to_string()) 
                            || exp_regex_white_space.is_match(&entrada.to_string()) 
                            && !exp_regex.is_match(&entrada.trim().to_string());

                    let aux_data_negative: bool = exp_regex_num_negative.is_match(&entrada.to_string());

                    /*  *** REGEX PARA LA VERSIÓN DE TIPOS FLOAT      */
                    let aux_data_incomplete: bool = exp_regex_data_incomplete.is_match(&entrada.to_string()) 
                            && !exp_regex_num_float.is_match(&entrada.to_string());

                        if aux_data_ok || aux_data_negative || aux_data_incomplete {
                            println!("\nError al introducir los Datos");
                        }
                           
                            
                        if !aux_data_ok || ( !aux_data_negative && !aux_data_incomplete ) {

                            if exp_regex_num_float.is_match(&entrada.to_string()) && !exp_regex_num_float_negative.is_match(&entrada.to_string()) {
                                punto_y_2 = f32::from_str(&entrada.trim()).unwrap();

                                data_ok = exp_regex_num_float.is_match(&punto_y_2.to_string()) 
                                    && !exp_regex_num_float_negative.is_match(&entrada.to_string());
            
                                    if data_ok {

                                        data_ok_all.push(true);

                                        break;
                                    }
                            }
                        }
                    
                }
        }

        if data_ok_all.len() == 4 {
            data_ok = is_all_data_ok(data_ok_all);

                if data_ok {
                    let _distancia_euclidea: f32 = get_distancia(punto_x_1, punto_y_1, punto_x_2, punto_y_2);

                    show_data_rangos(punto_x_1, punto_y_1, punto_x_2, punto_y_2);

                    println!("\nDistancia Euclídea:\t {}\n", _distancia_euclidea);

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

fn get_distancia(punto_x_1: f32, punto_y_1: f32, punto_x_2: f32, punto_y_2: f32) -> f32 {
    let result: f32;
    let auxs_x: f32;
    let auxs_y: f32;

        auxs_x = (punto_x_1 - punto_x_2).powf(2.0);
        auxs_y = (punto_y_1 - punto_y_2).powf(2.0);

        result = (auxs_x + auxs_y).sqrt();

    return result;
}

fn show_data_rangos(punto_x_1: f32, punto_y_1: f32, punto_x_2: f32, punto_y_2: f32) {

    println!("\nDatos Rango [ x_1: {}, y_1: {} ] [ x_2: {}, y_2: {} ]\n", punto_x_1, punto_y_1, punto_x_2, punto_y_2);

}

