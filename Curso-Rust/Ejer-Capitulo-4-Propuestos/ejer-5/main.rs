/*      Ejer    5       Diseñar una función a la que se le pasan las horas y minutos de dos instantes de tiempo, con el
                        prototipo.

                                fn diferencia_min ( hora1: i32, hora2: i32, min1: i32, min2: i32 )

                        La función devolverá la cantidad de minutos que existen de diferencia entre los dos instantes
                        utilizados.
*/

            /*  ***     TO DO       
            */

use std::io;
use std::str::FromStr;
use regex::Regex;

fn main() {
    let mut hora_1 : i32;
    let mut min_1 : i32;
    let mut hora_2 : i32;
    let mut min_2 : i32;
                        
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
            println!("\nIntroduce Hora Instante 1 [ ENTERO ] Formato [ 24 H ]");
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

                                hora_1 = i32::from_str(&entrada.trim()).unwrap();
                
                                data_ok = exp_regex.is_match(&hora_1.to_string());

                                /*  *** AJUSTES EN LA RECEPCIÓN DE DATOS 00 || 0     */
                    
                                    if data_ok && (hora_2 <= 23 && hora_2 >= 0) {
                
                                        data_ok_all.push(true);
                
                                        break;
                                    }
                            }
                        }
                    
                }
        }


        loop {
            println!("\nIntroduce Minutos Instante 1 [ ENTERO ] Formato [ 24 H ]");
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

                                min_1 = i32::from_str(&entrada.trim()).unwrap();
                
                                data_ok = exp_regex.is_match(&min_1.to_string());

                                /*  *** AJUSTES EN LA RECEPCIÓN DE DATOS 00 || 0  && <= 59   */
                    
                                    if data_ok && (min_1 <= 59 && min_1 >= 0) {
                
                                        data_ok_all.push(true);
                
                                        break;
                                    }
                            }
                        }
                    
                }
        }
        

        loop {
            println!("\nIntroduce Hora Instante 2 [ ENTERO ] Formato [ 24 H ]");
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

                                hora_2 = i32::from_str(&entrada.trim()).unwrap();
                
                                data_ok = exp_regex.is_match(&hora_2.to_string());

                                /*  *** AJUSTES EN LA RECEPCIÓN DE DATOS 00 || 0     */
                    
                                    if data_ok && (hora_2 <= 23 && hora_2 >= 0) {
                
                                        data_ok_all.push(true);
                
                                        break;
                                    }
                            }
                        }
                    
                }
        }


        loop {
            println!("\nIntroduce Hora Instante 1 [ ENTERO ] Formato [ 24 H ]");
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

                                min_2 = i32::from_str(&entrada.trim()).unwrap();
                
                                data_ok = exp_regex.is_match(&min_2.to_string());

                                /*  *** AJUSTES EN LA RECEPCIÓN DE DATOS 00 || 0  && <= 59   */
                    
                                    if data_ok && (min_1 <= 59 && min_1 >= 0) {
                
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

                    let _min_totales: i32 = get_diferencia_min(hora_1, min_1, hora_2, min_2);

                    let _instante_1: String = get_instante_tiempo(hora_1, min_1);

                    let _instante_2: String = get_instante_tiempo(hora_2, min_2);


                    show_instante_tiempo(_instante_1);

                    show_instante_tiempo(_instante_2);

                    println!("\nCantidad de Minutos Totales:\t {}\tseg\n", _min_totales);

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

fn get_minutos_horas(horas: i32) -> i32 {
    return horas * 60;
}

fn get_diferencia_min_horas(hora_1: i32, hora_2: i32) -> i32 {
    let result: i32;

        if (hora_1 > hora_2) && hora_1 <= 23  {
            result = get_minutos_horas(hora_2 + 1);
        }

        if (hora_1 < hora_2) && hora_2 <= 23 {
            result = get_minutos_horas(hora_2 - hora_1);
        }

        if (hora_1 <= 23) && hora_2 >= 0 {

            if hora_1 == 0 && hora_2 == 0 {
                result = 0;
            }

            if hora_2 == 0 {
                let aux_horas: i32 = 24 - hora_1;

                result = get_minutos_horas(aux_horas);
            } else {
                let aux_horas: i32 = (24 - hora_1) + hora_2;

                result = get_minutos_horas(aux_horas);
            }
        } 

    return result;
}

fn get_minutos_mins(min_1: i32, min_2: i32) -> i32 {
    let result: i32;

        if min_1 > min_2 {

            result = min_1 - min_2;
        } else if min_1 < min_2 {

            result = min_2 - min_1;
        } else {
            result = 0;
        }

    return result;
}


fn get_diferencia_min(hora_1: i32, min_1: i32, hora_2: i32, min_2: i32) -> i32 {

    let aux_result_min_horas: i32 = get_diferencia_min_horas(hora_1, hora_2);

    let aux_result_min_mins: i32 = get_minutos_mins(min_1, min_2);


    return aux_result_min_horas + aux_result_min_mins;
}


fn get_instante_tiempo(hora: i32, mins: i32) -> String {
    let mut result: String = String::new();

        result = "[ " + hora.to_string() + " : " + mins.to_string() + " ]";

    return result;
}


fn show_instante_tiempo(instante_tiempo: String) {
    println!("Instante de tiempo: {}\t", instante_tiempo);
}
