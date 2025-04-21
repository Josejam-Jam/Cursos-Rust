/*      Ejer 1          Diseñar una función que calcule la superficie y el volumen de una esfera.

                                Superficie = 4 PI * radio²
                                
                                Volumen = 4/3 PI * radio 3

*/

            /*  ***     TO DO       
            */

use std::io;
use std::str::FromStr;
use regex::Regex;

fn main() {
    let mut radio : f32;

    let exp_regex = Regex::new(r"\d+").unwrap();
    let exp_regex_caracter = Regex::new(r"[a-zA-Z]+").unwrap();
    let exp_regex_white_space = Regex::new(r"\s+").unwrap();

    let exp_regex_num_negative = Regex::new(r"(-\d+)||(-\d+\.\d+)").unwrap();

    /*  *** REGEX PARA LA VERSIÓN DE TIPOS FLOAT      */
    let exp_regex_data_incomplete = Regex::new(r"(\d+)[\,||\.||\']").unwrap();

    let exp_regex_num_float = Regex::new(r"\d+\.\d+").unwrap();
    let exp_regex_num_float_negative = Regex::new(r"-\d+\.\d+").unwrap();

    let mut data_ok: bool;
    let mut data_ok_all: Vec<bool> = Vec::new();


        loop {
            println!("\nIntroduce el radio [ Decimal - Float ]");
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
                                radio = f32::from_str(&entrada.trim()).unwrap();

                                data_ok = exp_regex_num_float.is_match(&radio.to_string()) && !exp_regex_num_float_negative.is_match(&entrada.to_string());
            
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

                    /*  *** VERSIÓN 1   CÁCULO, SALIDAS E IMPRESIÓN DE DATOS DE FORMA INDEPENDIENTE     */
                    let aux_superf: f32 = get_superficie_esfera(radio);
                    let aux_vol: f32 = get_volumen_esfera(radio);

                    /* ***  VERSIÓN 2   CÁLCULO FN DEVUELVE UNA TUPLA CON ( SUPERF, VOL )       */ 
                    let aux_tupla: (f32, f32) = get_superficie_and_volumen_esfera(radio);

                    println!("\nCálculo Superficie y Volumen Esfera \tradio = {}\n", radio);

                    /*  *** SHOW DATA VERSIÓN 1     */
                    println!("\nSuperficie:\t{}\tVolumen:\t{}\n", aux_superf, aux_vol);

                    /*  *** SHOW DATA VERSIÓN 2     */
                    show_superf_and_volumen_esfera_tupla(aux_tupla);

                    /*  *** SHOW DATA VERSIÓN 3     ( CÁLCULO DE DATOS E IMPRESIÓN EN LA MISMA FUNCIÓN )    */
                    show_superf_and_volumen_esfera(radio);

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

fn get_superficie_esfera(radio: f32) -> f32 {
    //  Superficie = 4 PI * radio²

    const PI:f32 = std::f32::consts::PI;

    return (4.0 * PI) * radio.powf(2.0);
}

fn get_volumen_esfera(radio: f32) -> f32 {
    //  Volumen = 4/3 PI * radio 3

    const PI:f32 = std::f32::consts::PI;

    return ((4.0 / 3.0) * PI) * radio.powf(3.0);
}

fn get_superficie_and_volumen_esfera(radio: f32) -> (f32,f32) {

    let aux_superf: f32 = get_superficie_esfera(radio);

    let aux_vol: f32 = get_volumen_esfera(radio);

    let result_sup_vol: (f32,f32) = (aux_superf, aux_vol);


    return result_sup_vol;

}

fn show_superf_and_volumen_esfera(radio: f32) {
    let result_tupla: (f32,f32) = get_superficie_and_volumen_esfera(radio);

    println!("\nSuperficie:\t{}\nVolumen:\t{}\n", result_tupla.0, result_tupla.1);
}

fn show_superf_and_volumen_esfera_tupla(tupla_sup_vol: (f32, f32)) {
    println!("\nSuperficie:\t{}\nVolumen:\t{}\n", tupla_sup_vol.0, tupla_sup_vol.1);
}

