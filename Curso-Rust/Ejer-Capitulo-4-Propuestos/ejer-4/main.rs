/*      Ejer    4       Escribir una función ( fn ) a la que se pasen como parámetros de entrada 
                        una cantidad de días, horas y minutos. 
                        
                        La función calculará y devolverá el número de segundos que existen en
                        los datos de entrada.
*/

            /*  ***     TO DO       
            */

use std::io;
use std::str::FromStr;
use regex::Regex;
                                    
fn main() {
    let mut dias : i32;
    let mut horas : i32;
    let mut mins : i32;
                        
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
            println!("\nIntroduce Días  [ ENTERO ]");
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
                            && exp_regex_num_float.is_match(&entrada.to_string());
                
                        if aux_data_ok || aux_data_negative || aux_data_incomplete {
                            println!("\nError al introducir los Datos");
                        }
                        
                        if !aux_data_ok || ( !aux_data_negative && !aux_data_incomplete) {
                
                            if exp_regex.is_match(&entrada.to_string()) && !exp_regex_num_float.is_match(&entrada.to_string()) 
                                    && !exp_regex_num_float_negative.is_match(&entrada.to_string()) {
                                dias = i32::from_str(&entrada.trim()).unwrap();
                
                                data_ok = exp_regex.is_match(&dias.to_string());
                    
                                    if data_ok {
                
                                        data_ok_all.push(true);
                
                                        break;
                                    }
                            }
                        }
                    
                }
        }

        loop {
            println!("\nIntroduce Horas  [ ENTERO ]");
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
                            && exp_regex_num_float.is_match(&entrada.to_string());
        
                        if aux_data_ok || aux_data_negative || aux_data_incomplete {
                            println!("\nError al introducir los Datos");
                        }
                        
                        if !aux_data_ok || ( !aux_data_negative && !aux_data_incomplete) {
                
                            if exp_regex.is_match(&entrada.to_string()) && !exp_regex_num_float.is_match(&entrada.to_string()) 
                                    && !exp_regex_num_float_negative.is_match(&entrada.to_string()) {
                                horas = i32::from_str(&entrada.trim()).unwrap();
                
                                data_ok = exp_regex.is_match(&horas.to_string());
                    
                                    if data_ok {
                
                                        data_ok_all.push(true);
                
                                        break;
                                    }
                            }
                        }
                    
                }
        }

        loop {
            println!("\nIntroduce Minutos [ ENTERO ]");
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
                            && exp_regex_num_float.is_match(&entrada.to_string());
        
                        if aux_data_ok || aux_data_negative || aux_data_incomplete {
                            println!("\nError al introducir los Datos");
                        }
                        
                        if !aux_data_ok || ( !aux_data_negative && !aux_data_incomplete) {
                
                            if exp_regex.is_match(&entrada.to_string()) && !exp_regex_num_float.is_match(&entrada.to_string()) 
                                    && !exp_regex_num_float_negative.is_match(&entrada.to_string()) {
                                mins = i32::from_str(&entrada.trim()).unwrap();
                
                                data_ok = exp_regex.is_match(&mins.to_string());
                    
                                    if data_ok {
                
                                        data_ok_all.push(true);
                
                                        break;
                                    }
                            }
                        }
                    
                }
        }
        
        if data_ok_all.len() == 3 {
            data_ok = is_all_data_ok(data_ok_all);

                if data_ok {

                    let _seconds_totales: i32 = get_seconds_all(dias, horas, mins);

                    println!("\nCantidad de Segundos Totales:\t {}\tseg\n", _seconds_totales);

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

fn get_seconds_dias(dias: i32) -> i32 {
    return ((dias) * 24) * 3600; 
}

fn get_seconds_horas(horas: i32) -> i32 {
    return horas * 3600;
}

fn get_seconds_minutos(mins: i32) -> i32 {
    return mins * 60;
}

fn get_seconds_all(dias: i32, horas: i32, mins: i32) -> i32 {
    let mut result: i32;

        result = get_seconds_dias(dias);
        result += get_seconds_horas(horas);
        result += get_seconds_minutos(mins);

    return result;
}
