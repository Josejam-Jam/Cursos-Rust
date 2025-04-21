/*      Ejer    3       Realizar una función ( fun ) que calcule y muestre el área o el volumen de un cilindro, según se
                        especifique. 
                        
                        Para distinguir un caso de otro se le pasará un número 1 ( para área ) o 2 ( para el volumen )

                        Además, hemos de pasarle a la función el radio de la base y la altura 

                        
                            volumen cilindro = π * r² * h

                            área Superficie Total Cilindro = 2π * r * h + 2π * r²

                            área Total Cilindro = 2π * r * (h + r) 



*/

use std::io;
use std::str::FromStr;
use regex::Regex;
// use compute_pi::compute_pi;      //  Add package como root cargo add [ compute_pi || compute_pi_str ]  se selecciona la cantidad de dígitos a complilar

fn main() {
    let mut radio_base: f32;
    let mut altura: f32;

    let exp_regex = Regex::new(r"\d+").unwrap();
    let exp_regex_caracter = Regex::new(r"[a-zA-Z]+").unwrap();
    let exp_regex_white_space = Regex::new(r"\s+").unwrap();

    let mut data_ok: bool;
    let mut data_ok_all: Vec<bool> = Vec::new();
 
        loop {
            println!("\nIntroduce Radio Base [ Decimales ]");
            let mut entrada = String::new();

            io::stdin().read_line(&mut entrada).expect("Error en la lectura de datos");

            let aux_ok = &entrada.is_empty();

                if !*aux_ok {
                    let aux_data_ok: bool = exp_regex_caracter.is_match(&entrada.trim().to_string()) || exp_regex_white_space.is_match(&entrada.to_string()) && !exp_regex.is_match(&entrada.trim().to_string());

                    if aux_data_ok {
                        println!("\nError al introducir los Datos");
                    }

                    if exp_regex.is_match(&entrada.trim().to_string()) {
                        radio_base = f32::from_str(&entrada.trim()).unwrap();

                        data_ok = exp_regex.is_match(&radio_base.to_string()) && radio_base > 0.0;

                            if data_ok {
                                data_ok_all.push(true);

                                break;
                            }
                    }
                }
        }

        loop {
            println!("\nIntroduce Altura [ Decimales ]");
            let mut entrada = String::new();

            io::stdin().read_line(&mut entrada).expect("Error en la lectura de datos");

            let aux_ok = &entrada.is_empty();

                if !*aux_ok {
                    let aux_data_ok: bool = exp_regex_caracter.is_match(&entrada.trim().to_string()) || exp_regex_white_space.is_match(&entrada.to_string()) && !exp_regex.is_match(&entrada.trim().to_string());

                    if aux_data_ok {
                        println!("\nError al introducir los Datos");
                    }

                    if exp_regex.is_match(&entrada.trim().to_string()) {
                        altura = f32::from_str(&entrada.trim()).unwrap();

                        data_ok = exp_regex.is_match(&altura.to_string()) && altura > 0.0;

                            if data_ok {
                                data_ok_all.push(true);

                                break;
                            }
                    }
                }
        }

        if data_ok_all.len() == 2 {
            data_ok = is_all_data_ok(data_ok_all);

                if data_ok {

                    let mut aux_opt : i32;

                        loop {
                            print_submenu_data_cilindro();

                            println!("\nIntroduce una de las Opciones del Ménu [ 1 || 2 ]");
                            let mut entrada = String::new();

                            io::stdin().read_line(&mut entrada).expect("Error en la lectura de datos");

                            let aux_ok = &entrada.is_empty();
                            let aux_exp_regex = Regex::new(r"\d").unwrap();

                                if !*aux_ok {
                                    let aux_data_ok: bool = exp_regex_caracter.is_match(&entrada.trim().to_string()) || exp_regex_white_space.is_match(&entrada.to_string()) && !aux_exp_regex.is_match(&entrada.trim().to_string());

                                    if aux_data_ok {
                                        println!("\nError al introducir los Datos");
                                    }

                                    if aux_exp_regex.is_match(&entrada.trim().to_string()) {
                                        aux_opt = i32::from_str(&entrada.trim()).unwrap();

                                        data_ok = aux_exp_regex.is_match(&aux_opt.to_string()) && (aux_opt == 1 || aux_opt == 2);

                                            if data_ok {

                                                if aux_opt == 1 {
                                                    println!("\nVolumen del Cilindro\t{}\n", get_volumen_cilindro(radio_base, altura));
                                                }

                                                if aux_opt == 2 {
                                                    println!("\nÁrea del Cilindro\t{}\n", get_area_total_cilindro(radio_base, altura));
                                                }

                                                break;
                                            }
                                    }
                                }
                        }

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

fn print_submenu_data_cilindro() {
    println!("\n\t\t\tMENÚ\n\n" );
    println!("\t1.\tCalcula Volumen del Cilintro");
    println!("\t2.\tCalcula Área del Cilintro\n\n");
}

fn get_volumen_cilindro(radio_base: f32, altura: f32) -> f32 {
    // volumen cilindro = π * r² * h

    const PI:f32 = std::f32::consts::PI;
    // let result : f32 = PI * radio_base.powf(2.0) * altura;

    return PI * radio_base.powf(2.0) * altura;
}

fn get_area_total_cilindro(radio_base: f32, altura: f32) -> f32 {
    //  área Total Cilindro = 2π * r * (h + r) 

    const PI:f32 = std::f32::consts::PI;
    // let result : f32 = (2.0 * PI)  * radio_base * (altura + radio_base);

    return (2.0 * PI)  * radio_base * (altura + radio_base);
}
