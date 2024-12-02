/*      Ejer 13     Dadas 6 notas escribir la cantidad de alumnos aprobados, condicionados ( = 4 ), y suspensos.

*/

use std::io;
use std::str::FromStr;
use regex::Regex;

fn main() {
    const NUM_MAX_CALIFICACIONES: i32 = 6;
    // let mut nota_int: i32;
    let mut nota_float: f32;
    // let mut array_calificaciones: [bool;5];
    // let mut vec_calificaciones = vec![false, false, false, false, false];
    let mut vec_calificaciones_suspensos: Vec<bool> = Vec::new();
    let mut vec_calificaciones_condicionados: Vec<bool> = Vec::new();
    let mut vec_calificaciones_aprobados: Vec<bool> = Vec::new();
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

                                    // let condicionado: bool = is_condicionado(nota_int); 
                                    let condicionado: bool = is_condicionado_float(nota_float);

                                    // let aprobado: bool = is_aprobado(nota_int); 
                                    let aprobado: bool = is_aprobado_float(nota_float);

                                        if suspenso {  vec_calificaciones_suspensos.push(suspenso);  }

                                        if condicionado {  vec_calificaciones_condicionados.push(condicionado);  } 

                                        if aprobado {  vec_calificaciones_aprobados.push(aprobado);  } 


                                    iterador += 1;

                                        if iterador == NUM_MAX_CALIFICACIONES {
                                            // show_result_suspensos(vec_calificaciones_suspensos);
                                            show_result_cant_suspensos(vec_calificaciones_suspensos);

                                            // show_result_condicionados(vec_calificaciones_condicionados);
                                            show_result_cant_condicionados(vec_calificaciones_condicionados);

                                            // show_result_aprobados(vec_calificaciones_aprobados);
                                            show_result_cant_aprobados(vec_calificaciones_aprobados);

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

/*  CASUÍSTICA NOTA TYPE: INT I32   */
/* fn is_condicionado(nota: i32) -> bool {
    return nota = 4;
}
*/

/*  CASUÍSTICA NOTA TYPE: INT f32   */
fn is_condicionado_float(nota: f32) -> bool {
    return nota >= 4.0 && nota < 5.0;
}

/*  CASUÍSTICA NOTA TYPE: INT I32   */
/* fn is_aprobado(nota: i32) -> bool {
    return nota >= 5;
}
*/

/*  CASUÍSTICA NOTA TYPE: INT f32   */
fn is_aprobado_float(nota: f32) -> bool {
    return nota >= 5.0;
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

/*  CASUÍSTICA  RETURN ONLY IF EXIST CONDICIONADOS  [ TRUE || FALSE ]   */
/* fn get_have_condicionados(vec_data: Vec<bool>) -> bool {
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

/*  CASUÍSTICA  RETURN ONLY IF EXIST APROBADOS  [ TRUE || FALSE ]   */
/* fn get_have_aprobados(vec_data: Vec<bool>) -> bool {
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

/*  CASUÍSTICA  RETURN ONLY IF EXIST CONDICIONADOS  [ TRUE || FALSE ]   */
fn get_cant_condicionados(vec_data: Vec<bool>) -> i32 {
    let mut result = 0;

        for i in 0..vec_data.len() {
            if vec_data[i] {
                result += 1;
            }
        }

    return result;
}

/*  CASUÍSTICA  RETURN ONLY IF EXIST APROBADOS  [ TRUE || FALSE ]   */
fn get_cant_aprobados(vec_data: Vec<bool>) -> i32 {
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

/* fn show_result_condicionados(vec_data: Vec<bool>) {
    let mut aux_condicionados: bool;

    aux_condicionados = get_have_condicionados(vec_data);

        if aux_condicionados {
            println!("\nSí hay algún Condicionado\n");
        } else {
            println!("\nNO hay Condicionados\n");
        }

}
*/

/* fn show_result_aprobados(vec_data: Vec<bool>) {
    let mut aux_aprobados: bool;

    aux_aprobados = get_have_aprobados(vec_data);

        if aux_aprobados {
            println!("\nSí hay algún Aprobado\n");
        } else {
            println!("\nNO hay Aprobados\n");
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

/*  CASUÍSTICA  RETURN CANTIDAD IF EXIST CONDICIONADOS  */
fn show_result_cant_condicionados(vec_data: Vec<bool>) {
    let aux_cant_condicionados: i32;

    aux_cant_condicionados = get_cant_condicionados(vec_data);

        if aux_cant_condicionados > 0 {
            println!("\nSí Hay Condicionados, hay {}\tCondicionados\n", aux_cant_condicionados);
        } else {
            println!("\nNO hay Condicionados\n");
        }
}

/*  CASUÍSTICA  RETURN CANTIDAD IF EXIST APROBADOS  */
fn show_result_cant_aprobados(vec_data: Vec<bool>) {
    let aux_cant_aprobados: i32;

    aux_cant_aprobados = get_cant_aprobados(vec_data);

        if aux_cant_aprobados > 0 {
            println!("\nSí Hay Aprobados, hay {}\tAprobados\n", aux_cant_aprobados);
        } else {
            println!("\nNO hay Aprobados\n");
        }
}
