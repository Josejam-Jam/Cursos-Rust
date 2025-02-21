/*      Ejer  4     Calcular la raíz cuadrada de un número natural mediante aproximaciones.
                    
                    En el caso de que no sea exacta, mostraremos el resto.

                    Por ejemplo, para calcular raíz( 23 ), probamos 1² = 1, 2² = 4, 3² = 9, 4² = 16, 5² = 25 ( nos
                    pasamos ), resultando 4 la raíz cuadrada de 23 con un resto (23 – 16) de 7.
*/

use std::io;
use std::str::FromStr;
use regex::Regex;

fn main() {
    let mut num: f32;
 
    let exp_regex = Regex::new(r"\d+").unwrap();
    let exp_regex_caracter = Regex::new(r"[a-zA-Z]+").unwrap();
    let exp_regex_white_space = Regex::new(r"\s+").unwrap();

    let mut data_ok : bool;

    loop {
        println!{"\nIntroduce un número para calcular la Raíz Cuadrada"};
        let mut entrada = String::new();

        io::stdin().read_line(&mut entrada).expect("Error en la lectura de datos");

        let aux_ok = &entrada.is_empty();

            if !*aux_ok  {
                let aux_data_ok: bool = exp_regex_caracter.is_match(&entrada.trim().to_string()) || exp_regex_white_space.is_match(&entrada.to_string()) && !exp_regex.is_match(&entrada.trim().to_string());

                    if aux_data_ok {
                        println!("\nError al introducir los Datos");
                    }

                    if exp_regex.is_match(&entrada.trim().to_string()) {

                        num = f32::from_str(&entrada.trim()).unwrap();

                        data_ok = exp_regex.is_match(&num.to_string()) && num >= 0.0;

                            if data_ok {
                                break;
                            }
                    }

            }

    }

    if data_ok {

        let raiz_cuadrada_tupla: (i32, f32) = get_raiz_cuadrada_aprox(num);

        let raiz_cuadrada: i32 = raiz_cuadrada_tupla.0;

        let raiz_resto: i32 = get_rest_raiz(raiz_cuadrada_tupla.1);

        let men_result: String = get_mensaje_cuadrada_aprox(raiz_cuadrada, raiz_resto);

        print_raiz_cuadrada_aprox(men_result);
    }
    
}

fn get_rest_regulacion_redondeo(rest: f32) -> i32 {
    let mut result_rest : i32 = 0;
    
        if rest > 0.0 && rest <= 0.9 {
            result_rest = (rest* 10.0)  as i32;
        }

        if rest > 0.01 && rest <= 0.09 {
            result_rest = 1;
        }

    return result_rest;
}

fn get_rest_raiz(rest: f32) -> i32 {

    return get_rest_regulacion_redondeo(rest);
}

/*
fn get_raiz_cuadrada_aprox(num: i32) -> i32 {

}
*/

fn get_raiz_cuadrada_aprox(num: f32) -> (i32, f32) {

    let result_raiz : f32 = num.sqrt();
    let mut aux_rest = result_raiz - result_raiz.trunc();

    let aux_raiz: i32 = result_raiz.trunc() as i32;

        if aux_rest == 0.0 {
            aux_rest = aux_rest as f32;
        }
    
    let result_tupla : (i32, f32) =  (aux_raiz, aux_rest);

    return result_tupla;

}

fn get_mensaje_cuadrada_aprox(result_raiz: i32, rest: i32) -> String {
    let result_men: String;

    let aux_men_rest: String;

        if rest == 0  {
            aux_men_rest = "".to_string();
        } else {
            aux_men_rest = "con un resto de ".to_string() + &rest.to_string();
        }

        result_men = "Resultado\t".to_string() + &result_raiz.to_string() + "\t" + &aux_men_rest.to_string();

    return result_men;
}

fn print_raiz_cuadrada_aprox(mensaje_result: String) {
    println!("\n{}\n", mensaje_result);
}
