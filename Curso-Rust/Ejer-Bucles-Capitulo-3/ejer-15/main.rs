/*      Ejer    15      Realizar un programa que nos pida un número n, y nos diga cuántos número hay entre 1 y n
                        que sean primos. 
                        
                        Un número primo es aquél que solo es divisible por 1 y por él mismo.

                        Veamos un ejemplo para n = 8:

                        Comprobamos todos los números del 1 al 8

                            1 → primo

                            2 → primo

                            3 → primo

                            4 → no primo

                            5 → primo

                            6 → no primo

                            7 → primo

                            8 → no primo

                        Resultando un total de 5 números primos
 
*/

use std::io;
use std::str::FromStr;
use regex::Regex;

fn main() {
    let mut num: i32;

    let exp_regex = Regex::new(r"\d+").unwrap();
    let exp_regex_caracter = Regex::new(r"[a-zA-Z]+").unwrap();
    let exp_regex_white_space = Regex::new(r"\s+").unwrap();

    let mut data_ok: bool;


    loop {
        println!("\nIntroduce un Número N para Calcular/Mostrar los Números Primos entre [ 1 - N ]");
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
    
                        num = i32::from_str(&entrada.trim()).unwrap();
    
                        data_ok = exp_regex.is_match(&num.to_string()) && num > 0;
    
                            // if exp_regex.is_match(&num.to_string()) {

                            if data_ok {

                                show_traza_num_primos(num);

                                /* OPCIÓN 2: MENOS EFICIENTE YA QUE LAS FUNCIONES ESTÁN SEPARADAS EJECUTÁNDOSE UN BUCLE MÁS      */

                                /* show_result_total_num_primos(num);  */

                                break;

                            } else {
                                println!("\nError al introducir los Datos");
                            }
                    }
            }
    }

}

fn is_num_primo(num: i32) -> bool {
    let mut result:  bool = false;

        if num > 3 {
            result = num % 1 == 0 && num % 2 != 0 && num % num == 0;
        } else {

            if num > 0 && num <= 3 {
                result = true; 
            }
        }

    return result;
}

fn get_mensaje_primos(num: i32, result_eval: bool) -> String {

    let result: bool = result_eval;
    let result_men: String; 

        if result {
            result_men = num.to_string() + "\tPrimo"; 
        } else {
            result_men = num.to_string() + "\tNo Primo";
        }
    
    return result_men;
}

/*  OPCIÓN 2:  */
/* fn get_cant_total_num_primos(num: i32) -> i32 {
    let mut aux_iterador: i32 = 0;
    let mut contador: i32 = 0;
    let mut result_eval: bool;

        loop {

            aux_iterador += 1;

            result_eval = is_num_primo(aux_iterador);

                if result_eval { contador += 1;  }

                if aux_iterador == num {    break;    }
            
        }
    return contador;
}
*/

fn show_traza_num_primos(num: i32) {
    let mut aux_iterador: i32 = 0;
    let mut contador: i32 = 0;
    let mut result_eval: bool;
    let mut result_men: String;

        loop {

            aux_iterador += 1;

            result_eval = is_num_primo(aux_iterador);

                if result_eval { contador += 1;  }

            result_men = get_mensaje_primos(aux_iterador, result_eval);

            println!("\n{}", result_men);

                if aux_iterador == num {    break;    }
            
        }

    println!("\nResultado\t {} Números Primos\n", contador);
}

/*  OPCIÓN 2:     */
/* fn show_result_total_num_primos(num: i32) {
    let result: i32 = get_cant_total_num_primos(num);

    println!("\nResultado\t {} Números Primos\n", result);
}
*/    
