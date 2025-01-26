/*      Ejer    12      Pedir 5 calificaciones de alumnos y decir al final si hay algún suspenso.

*/

use std::io;
use std::str::FromStr;
use regex::Regex;

fn main() {
    const NUM_MAX_CALIFICACIONES: i32 = 5;
    // let mut nota_int: i32;
    let mut nota_float: f32;
    // let mut array_calificaciones: [bool;5];
    // let mut vec_calificaciones = vec![false, false, false, false, false];
    let mut vec_calificaciones: Vec<bool> = Vec::new();
    let exp_regex = Regex::new(r"\d+").unwrap();
    let exp_regex_caracter = Regex::new(r"[a-zA-Z]+").unwrap();
    let exp_regex_white_space = Regex::new(r"\s+").unwrap();

    let mut data_ok: bool;
    let mut iterador: i32 = 0;

        loop {
            println!("\nIntroduce Calificación del Alumno");
            let mut entrada = String::new();

            io::stdin().read_line(&mut entrada).expect("Error en la lectura de datos");

            let aux_ok = &entrada.is_empty();

                if !*aux_ok {

                    let aux_data_ok:bool = exp_regex_caracter.is_match(&entrada.trim().to_string()) || exp_regex_white_space.is_match(&entrada.to_string()) && !exp_regex.is_match(&entrada.trim().to_string());
                    // let aux_data_ok:bool = exp_regex_caracter.is_match(&entrada.trim().to_string());

                    // if exp_regex_caracter.is_match(&entrada.trim().to_string()) {
                        if aux_data_ok {
                            println!("\nError al introducir los Datos Introduzca un número");
                        } 
                        
                        if exp_regex.is_match(&entrada.trim().to_string())  {
        
                            // nota_int = i32::from_str(&entrada.trim()).unwrap();
                            nota_float = f32::from_str(&entrada.trim()).unwrap();
        
                            // data_ok = exp_regex.is_match(&nota_int.to_string());
                            data_ok = exp_regex.is_match(&nota_float.to_string());
        
                                // if exp_regex.is_match(&nota_int.to_string()) {
                                // if exp_regex.is_match(&nota_float.to_string()) {

                                if data_ok {
                                    
                                    // let suspenso: bool = is_suspenso(nota_int); 
                                    let suspenso: bool = is_suspenso_float(nota_float); 

                                    vec_calificaciones.push(suspenso);

                                    //vec_calificaciones[iterador] = suspenso;

                                    iterador += 1;

                                        if iterador == NUM_MAX_CALIFICACIONES {
                                            // show_result_suspensos(vec_calificaciones);
                                            show_result_cant_suspensos(vec_calificaciones);

                                            break;
                                        }
                                    
                                } else {
                                    println!("\nError al introducir los Datos");
                                }
                        }
                }
        }

}

/*  CASUÍSTICA  NOTA TYPE: INT  I32   */
/* fn is_suspenso(nota: i32) -> bool {
    return nota < 5;
}
*/    

/*  CASUÍSTICA  NOTA TYPE: FLOAT  F32   */
fn is_suspenso_float(nota: f32) -> bool {
    return nota < 5.0;
}

/*  CASUÍSTICA  RETURN ONLY IF EXIST SUSPENSOS  [ TRUE || FALSE ]   */
/* fn get_have_suspensos(vec_data: Vec<bool>) -> bool {
    let mut result = false;
        
        /* for i in 0..vec_data.length() {
            if vec_data[i] {
                result = true;
            }
        }
        */

        for i in 0..vec_data.len() {

            if vec_data[i] {
                result = true;
            }
        }

    return result;
}
*/

/*  CASUÍSTICA  RETURN ONLY IF EXIST SUSPENSOS  [ TRUE || FALSE ]   */
fn get_cant_suspensos(vec_data: Vec<bool>) -> i32 {
    let mut result = 0;

        for i in 0..vec_data.len() {
            if vec_data[i] {
                result += 1;
            }
        }

    return result;
}

/*  CASUÍSTICA  SHOW ONLY IF EXIST SUSPENSOS   */
/* fn show_result_suspensos(vec_data: Vec<bool>) {
    let mut aux_suspensos: bool;

    aux_suspensos = get_have_suspensos(vec_data);

        if aux_suspensos {
            println!("\nSí hay algún Suspenso\n");
        } else {
            println!("\nNO hay Suspensos\n");
        }

}
*/        

/*  CASUÍSTICA  RETURN CANTIDAD IF EXIST SUSPENSOS  */
fn show_result_cant_suspensos(vec_data: Vec<bool>) {
    let aux_cant_suspensos: i32;

    aux_cant_suspensos = get_cant_suspensos(vec_data);

        if aux_cant_suspensos > 0 {
            println!("\nSí Hay Suspensos, hay {}\tsuspensos\n", aux_cant_suspensos);
        } else {
            println!("\nNO hay Suspensos\n");
        }
}