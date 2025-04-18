/*      Ejer    12      Diseñar la función ( fun ) calculadora ( num1: i32, num2: i32 ), a la que se pasan dos
                        números enteros ( operandos ) y qué operación se desea realizar con ellos. 

                        Las operaciones disponibles son: sumar, restar, multiplicar o dividir.
                        
                        Estas se especifican mediante un número: 1 para la suma, 2 para la resta, 3 para la
                        multiplicación y 4 para la división.
*/

            /*  *** TO DO   IMPRIMIR EL TIPO DE OPERACIÓN PREVIO A LA LLAMADA A LA FUNCIÓN CALCULADORA

                    DISEÑAR FUNCIÓN PARA LAS FUTURAS MODIFICACIONES Y/O AMPLIACIONES

                *** TO DO   REALIZAR UNA VERSIÓN PARA LA RECEPCIÓN DE PARÁMETROS TIPO FLOAT,
                            TANTO PARA LA ENTRADA DE DATOS, ASÍ COMO PARA LA PARAMETRIZACIÓN DE LAS FUNCIONES
                    
            */

use std::io;
use std::str::FromStr;
use regex::Regex;

fn main() {

    let mut operando_1 : i32;
    let mut operando_2 : i32;

    let mut num_operation : i32;

    let exp_regex = Regex::new(r"\d+").unwrap();
    let exp_regex_caracter = Regex::new(r"[a-zA-Z]+").unwrap();
    let exp_regex_white_space = Regex::new(r"\s+").unwrap();
    /*  *** REGEX PARA LA VERSIÓN DE TIPOS FLOAT      */
    /*let exp_regex_data_incomplete = Regex::new(r"(\d+)[\,||\.||\']").unwrap();

    let exp_regex_num_float = Regex::new(r"\d+\.\d+").unwrap();
    */

    let mut data_ok: bool;
    let mut data_ok_all: Vec<bool> = Vec::new();


        loop {
            println!("\nIntroduce un Número [ Operando 1 ]");
            let mut entrada = String::new();

            io::stdin().read_line(&mut entrada).expect("Error en la lectura de datos");

            let aux_ok = &entrada.is_empty();

                if !*aux_ok {
                    let aux_data_ok: bool = exp_regex_caracter.is_match(&entrada.trim().to_string()) 
                            || exp_regex_white_space.is_match(&entrada.to_string()) && !exp_regex.is_match(&entrada.trim().to_string());

                    if aux_data_ok  {
                        println!("\nError al introducir los Datos 1");
                    }

                    /*  *** REGEX PARA LA VERSIÓN DE TIPOS FLOAT      */
                    //let aux_data_incomplete: bool = exp_regex_data_incomplete.is_match(&entrada.to_string()) && !exp_regex_num_float.is_match(&entrada.to_string());

                    // if aux_data_incomplete {
                    //     println!("\nError al introducir los Datos 2");
                    // }

                    if !aux_data_ok {

                        if exp_regex.is_match(&entrada.trim().to_string()) {
                            operando_1 = i32::from_str(&entrada.trim()).unwrap();
    
                            data_ok = exp_regex.is_match(&operando_1.to_string());
        
                                if data_ok{
    
                                    data_ok_all.push(true);
    
                                    break;
                                }
                        }
                    }
                    
                }
        }

        loop {
            println!("\nIntroduce un Número [ Operando 2 ]");
            let mut entrada = String::new();

            io::stdin().read_line(&mut entrada).expect("Error en la lectura de datos");

            let aux_ok = &entrada.is_empty();

                if !*aux_ok {
                    let aux_data_ok: bool = exp_regex_caracter.is_match(&entrada.trim().to_string()) 
                            || exp_regex_white_space.is_match(&entrada.to_string()) && !exp_regex.is_match(&entrada.trim().to_string());

                    if aux_data_ok  {
                        println!("\nError al introducir los Datos 1");
                    }

                    /*  *** REGEX PARA LA VERSIÓN DE TIPOS FLOAT      */
                    // let aux_data_incomplete: bool = exp_regex_data_incomplete.is_match(&entrada.to_string()) && !exp_regex_num_float.is_match(&entrada.to_string());

                    // if aux_data_incomplete {
                    //     println!("\nError al introducir los Datos 2");
                    // }

                    if !aux_data_ok {

                        if exp_regex.is_match(&entrada.to_string()) {
                            operando_2 = i32::from_str(&entrada.trim()).unwrap();
    
                            data_ok = exp_regex.is_match(&operando_2.to_string());
        
                                if data_ok{
    
                                    data_ok_all.push(true);
    
                                    break;
                                }
                        }
                    }
                    
                }
        }

        loop {

            print_submenu_data_calculadora();

            println!("\nIntroduce un Número [ Operación ] [ 1 - 4 ]");
            let mut entrada = String::new();

            io::stdin().read_line(&mut entrada).expect("Error en la lectura de datos");

            let aux_ok = &entrada.is_empty();

                if !*aux_ok {
                    let aux_data_ok: bool = exp_regex_caracter.is_match(&entrada.trim().to_string()) 
                            || exp_regex_white_space.is_match(&entrada.to_string()) && !exp_regex.is_match(&entrada.trim().to_string());

                        if aux_data_ok  {
                            println!("\nError al introducir los Datos 1");
                        }

                        /*  *** REGEX PARA LA VERSIÓN DE TIPOS FLOAT      */
                        // let aux_data_incomplete: bool = exp_regex_data_incomplete.is_match(&entrada.to_string()) && !exp_regex_num_float.is_match(&entrada.to_string());

                        // if aux_data_incomplete {
                    //     println!("\nError al introducir los Datos 2");
                    // }

                        if !aux_data_ok {

                            if exp_regex.is_match(&entrada.to_string()) {
                                num_operation = i32::from_str(&entrada.trim()).unwrap();
        
                                data_ok = exp_regex.is_match(&num_operation.to_string());
            
                                    if data_ok && ( num_operation >= 1 && num_operation <= 4 )  {
        
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

                    calculadora(operando_1, operando_2, num_operation);

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

fn print_submenu_data_calculadora() {
    println!("\n\t\t\tMENÚ\n\n" );
    println!("\t1.\tCalcula Suma");
    println!("\t2.\tCalcula Resta");
    println!("\t3.\tCalcula Multiplicación");
    println!("\t4.\tCalcula División\n\n");
}

fn get_suma(operando_1: i32, operando_2: i32) -> i32 {
    return operando_1 + operando_2;
}

fn get_resta(operando_1: i32, operando_2: i32) -> i32 {
    return operando_1 - operando_2;
}

fn get_multiplicacion(operando_1: i32, operando_2: i32) -> i32 {
    return operando_1 * operando_2;
}

fn get_division(operando_1: i32, operando_2: i32) -> i32 {
    return operando_1 / operando_2;
}

fn print_details_operations(operando_1: i32, operando_2: i32, operation: i32) {
    let mut result: String = operando_1.to_string();

            if operation == 1 { 
                result = result + " + " + &operando_2.to_string() + " = ";
            }

            if operation == 2 {
                result = result + " - " + &operando_2.to_string() + " = ";
            }
            
            if operation == 3 {
                result = result + " * " + &operando_2.to_string() + " = ";
            }
            
            if operation == 4 { 
                result = result + " / " + &operando_2.to_string() + " = ";
            } 


    print!("{}", result);
}


fn calculadora(operando_1: i32, operando_2: i32, operation: i32) {
    let mut result: String = String::new();

        if operation == 1 {
            result = get_suma(operando_1, operando_2).to_string();
        }

        if operation == 2 {
            result = get_resta(operando_1, operando_2).to_string();
        }

        if operation == 3 {
            
            if operando_1 == 0 || operando_2 == 0 {
                result = "0".to_string();
            } else {
                result = get_multiplicacion(operando_1, operando_2).to_string();
            }
        }

        if operation == 4 {

            if operando_1 == 0 || operando_2 == 0 {
                if operando_1 == 0 && operando_2 >= 0 {
                    result = "Indeterminación".to_string();
                }

                if operando_2 == 0 && operando_1 >= 0 {
                    result = "Indeterminación".to_string();
                }      

            } else {
                result = get_division(operando_1, operando_2).to_string();
            }
            
        }

    println!("");    

    print_details_operations(operando_1, operando_2, operation);

    print!("\t{}\n", result.to_string());

    println!("\nResultado:\t{}\n", result);
}
