/*      Ejer 2      Escribe un programa que incremente la hora de un reloj. 

                    Se pedirán por teclado la hora, los minutos y segundos, 
                    así como cuantos segundos se desea incrementar la hora introducida. 
                    La aplicación mostrará la nueva hora.


                    Por ejemplo, si las 13:59:51 se incrementan en 10 segundos resultan las 14:00:01. 

*/

use std::io;
use std::str::FromStr;
use regex::Regex;

fn main() {
 
    let mut hora: i32;
    let mut min: i32;
    let mut seg: i32;
    let mut inc_seg: i32;

    let exp_regex = Regex::new(r"\d{2}").unwrap();
    let exp_regex_caracter = Regex::new(r"[a-zA-Z]+").unwrap();
    let exp_regex_white_space = Regex::new(r"\s+").unwrap();
    let exp_regex_inc = Regex::new(r"\d+").unwrap();

    let mut data_ok : bool;
    let mut data_ok_all : Vec<bool> = Vec::new();


        loop {

            println!("\nIntroduce horas, formato 24H [ 00-24 | HH:min:seg ]");
            let mut entrada = String::new();

            io::stdin().read_line(&mut entrada).expect("Error en la lectura de datos");

            let aux_ok = &entrada.is_empty();


                if !*aux_ok {
                    let aux_data_ok:bool = exp_regex_caracter.is_match(&entrada.trim().to_string()) || exp_regex_white_space.is_match(&entrada.to_string()) && !exp_regex.is_match(&entrada.trim().to_string());

                        if aux_data_ok {
                            println!("\nError al introducir los Datos");
                        }

                        if exp_regex.is_match(&entrada.trim().to_string()) {

                            hora = i32::from_str(&entrada.trim()).unwrap();

                            data_ok = exp_regex.is_match(&hora.to_string()) && hora >= 0 && hora <= 24;

                                if data_ok  {
                                    data_ok_all.push(true);
                                    
                                    break;
                                }
                        }
                }

        }

        loop {

            println!("\nIntroduce Minutos, formato 24H [ hh:MIN ]");
            let mut entrada = String::new();

            io::stdin().read_line(&mut entrada).expect("Error en la lectura de datos");

            let aux_ok = &entrada.is_empty();


                if !*aux_ok {
                    let aux_data_ok:bool = exp_regex_caracter.is_match(&entrada.trim().to_string()) || exp_regex_white_space.is_match(&entrada.to_string()) && !exp_regex.is_match(&entrada.trim().to_string());

                        if aux_data_ok {
                            println!("\nError al introducir los Datos");
                        }

                        if exp_regex.is_match(&entrada.trim().to_string()) {

                            min = i32::from_str(&entrada.trim()).unwrap();

                            data_ok = exp_regex.is_match(&min.to_string()) && min >= 0 && min <= 60;

                                if data_ok  {
                                    data_ok_all.push(true);
                                    
                                    break;
                                }
                        }
                }

        }

        loop {

            println!("\nIntroduce Segundos, formato 24H [ hh:min:SEG ]");
            let mut entrada = String::new();

            io::stdin().read_line(&mut entrada).expect("Error en la lectura de datos");

            let aux_ok = &entrada.is_empty();


                if !*aux_ok {
                    let aux_data_ok:bool = exp_regex_caracter.is_match(&entrada.trim().to_string()) || exp_regex_white_space.is_match(&entrada.to_string()) && !exp_regex.is_match(&entrada.trim().to_string());

                        if aux_data_ok {
                            println!("\nError al introducir los Datos");
                        }

                        if exp_regex.is_match(&entrada.trim().to_string()) {

                            seg = i32::from_str(&entrada.trim()).unwrap();

                            data_ok = exp_regex.is_match(&seg.to_string()) && seg >= 0;

                                if data_ok {
                                    data_ok_all.push(true);
                                    
                                    break;
                                }
                        }
                }

        }

        if data_ok_all.len() == 3 {

            data_ok = is_all_data_ok(data_ok_all);

                if data_ok {

                    loop {

                        println!("\nIntroduce Cantidad de Segundos a Incrementar, formato 24H [ hh:min:SEG ]");
                        let mut entrada = String::new();

                        io::stdin().read_line(&mut entrada).expect("Error en la lectura de datos");

                        let aux_ok = &entrada.is_empty();


                            if !*aux_ok {
                                let aux_data_ok:bool = exp_regex_caracter.is_match(&entrada.trim().to_string()) || exp_regex_white_space.is_match(&entrada.to_string()) && !exp_regex.is_match(&entrada.trim().to_string());

                                    if aux_data_ok {
                                        println!("\nError al introducir los Datos");
                                    }

                                    if exp_regex_inc.is_match(&entrada.trim().to_string()) {

                                        inc_seg = i32::from_str(&entrada.trim()).unwrap();

                                        data_ok = exp_regex_inc.is_match(&inc_seg.to_string()) && inc_seg >= 0;

                                            if data_ok {                                                
                                                break;
                                            }
                                    }
                            }

                    }

                    print_increment_segundos_all_hora(hora, min, seg, inc_seg);

                }

        }   

}

/*  REALIZAR EL MATCH  IF DATA-IN == FALSE */

fn is_all_data_ok(data_ok_all: Vec<bool>) -> bool{
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


fn get_increment_seg_tupla(mut set_tupla: (i32, i32, i32)) -> (i32, i32, i32) {

        if set_tupla.2 + 1 == 60 {
            set_tupla.1 += 1;
            set_tupla.2 = 0;
        } 
        
        if set_tupla.1 == 60 {
            set_tupla.0 += 1;
            set_tupla.1 = 0;

        }
        
        if set_tupla.0 > 23 {
            set_tupla.0 = 0;
        }

        set_tupla.2 += 1; 
        
    return set_tupla;

}


fn set_increment_segundos(hora: i32, min: i32, seg: i32, inc_seg: i32) -> (i32, i32, i32) {
    let mut result_tupla: (i32, i32, i32) = (hora, min, seg);

        for _i in 1..inc_seg  {
            result_tupla = get_increment_seg_tupla(result_tupla);
        }

    return result_tupla;
}

fn increment_segundos_all_hora(hora: i32, min: i32, seg: i32, inc_seg: i32) -> (i32, i32, i32) {


    let result_tupla: (i32, i32, i32) = set_increment_segundos(hora, min, seg, inc_seg);

        return result_tupla;
}

/*  *** CASUÍSTICA EN LA QUE SE IMPRIME DIRECTAMENTE DESDE LA TUPLA RESULTANTE      */
/*  fn print_incrementar_hora_tupla(result_tupla: (i32, i32, i32)) {
    // let result: (String, String, String);
    let aux_hora;
    let aux_min;
    let aux_seg;

        if result_tupla.0 == 0 || result_tupla.0 < 10 {
            aux_hora = "0".to_string() + &result_tupla.0.to_string();
        } else {
            aux_hora = result_tupla.0.to_string();
        }

        if result_tupla.1 == 0 || result_tupla.1 < 10 {
            aux_min = "0".to_string() + &result_tupla.1.to_string();
        } else {
            aux_min = result_tupla.1.to_string();
        }

        if result_tupla.2 == 0 || result_tupla.2 < 10 {
            aux_seg = "0".to_string() + &result_tupla.2.to_string();
        } else {
            aux_seg = result_tupla.2.to_string();
        }     

    println!("\nHora :\t{}:{}:{}\n", aux_hora, aux_min, aux_seg);

    /* 
    result = (hora, min,  seg);

    println!("\nHora :\t{}:{}:{}\n", result.0, result.1, result.2);
    */    
}
*/

fn print_increment_segundos_all_hora(hora: i32, min: i32, seg: i32, inc_seg: i32) {
    // let result: (String, String, String);
    let aux_hora;
    let aux_min;
    let aux_seg;

    let aux_tupla: (i32, i32, i32) = increment_segundos_all_hora(hora, min, seg, inc_seg);

        if aux_tupla.0 == 0 || aux_tupla.0 < 10 {
            aux_hora = "0".to_string() + &aux_tupla.0.to_string();
        } else {
            aux_hora = aux_tupla.0.to_string();
        }

        if aux_tupla.1 == 0 || aux_tupla.1 < 10 {
            aux_min = "0".to_string() + &aux_tupla.1.to_string();
        } else {
            aux_min = aux_tupla.1.to_string();
        }

        if aux_tupla.2 == 0 || aux_tupla.2 < 10 {
            aux_seg = "0".to_string() + &aux_tupla.2.to_string();
        } else {
            aux_seg = aux_tupla.2.to_string();
        }     

    println!("\nHora :\t{}:{}:{}\n", aux_hora, aux_min, aux_seg);

    /* 
    result = (hora, min,  seg);

    println!("\nHora :\t{}:{}:{}\n", result.0, result.1, result.2);
    */  
}
