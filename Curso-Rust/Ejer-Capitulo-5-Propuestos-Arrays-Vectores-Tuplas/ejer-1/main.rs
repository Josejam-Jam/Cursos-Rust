/*      Ejer    1       El ayuntamiento de nuestra localidad nos ha encargado una aplicación que ayude a realizar
                        encuestas estadísticas para conocer el nivel adquisitivo de los habitantes del municipio. 
                        Para ello, tendremos que preguntar el sueldo a cada persona encuestada.

                        A priori no conocemos el número de encuestados. Para finalizar la entrada de datos,
                        introduciremos un sueldo con valor -1.

                        Una vez terminada la entrada de datos, hemos de mostrar la siguiente información:

                            ● Todos los sueldos introducidos ordenados de forma decreciente.

                            ● El sueldo máximo y mínimo.

                            ● La media de los sueldos.

                        Nota: Como a priori se desconoce el número de sueldos, la tabla donde se almacenan los datos

                        tendrá que incrementar su tamaño conforme necesitemos más espacios.
*/

            /*  ***     TO DO       
            */

use std::io;
use std::str::FromStr;
use regex::Regex;

fn main() {
    //let salarios_all : [f32; 10];
    //let salarios_all;
    let mut salarios_all: Vec<f32> = Vec::new();
    //let aux_lenght: usize = 10;
    let mut data_salario : f32;
                        
    let exp_regex = Regex::new(r"\d+").unwrap();
    let exp_regex_caracter = Regex::new(r"[a-zA-Z]+").unwrap();
    let exp_regex_white_space = Regex::new(r"\s+").unwrap();
                        
    let exp_regex_num_negative = Regex::new(r"(-\d+)").unwrap();
                        
    /*  *** REGEX PARA LA VERSIÓN DE TIPOS FLOAT      */
    let exp_regex_data_incomplete = Regex::new(r"(\d+)[\,||\.||\']").unwrap();
                        
    let exp_regex_num_float = Regex::new(r"\d+\.\d+").unwrap();
    let exp_regex_num_float_negative = Regex::new(r"-\d+\.\d+").unwrap();

    let exp_regex_salida = Regex::new(r"-1").unwrap();
                        
    let mut data_ok: bool;
    let mut data_ok_all: Vec<bool> = Vec::new();

        loop {
            println!("\nIntroduce X - 1 [ Float ]");
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

                    let salida_bucle: bool = exp_regex_salida.is_match(&entrada.to_string());        

                        if aux_data_negative && salida_bucle {  break;  }  

                        if aux_data_ok || aux_data_negative || aux_data_incomplete {
                            println!("\nError al introducir los Datos");
                        }
                        
                    
                           
                        if !aux_data_ok || ( !aux_data_negative && !aux_data_incomplete ) {

                            if exp_regex_num_float.is_match(&entrada.to_string()) && !exp_regex_num_float_negative.is_match(&entrada.to_string()) {
                                data_salario = f32::from_str(&entrada.trim()).unwrap();

                                data_ok = exp_regex_num_float.is_match(&data_salario.to_string()) 
                                    && !exp_regex_num_float_negative.is_match(&entrada.to_string());
            
                                    if data_ok {

                                        /*if data_salario == -1.0 {
                                            break;
                                        }*/

                                        data_ok_all.push(true);

                                        salarios_all.push(data_salario);

                                    }
                            }
                        }
                    
                }
        }

        if data_ok_all.len() > 0 && (data_ok_all.len() == salarios_all.len()) {

            data_ok = is_all_data_ok(data_ok_all);

            if data_ok {

                show_salarios(salarios_all.clone());

                let mut sort_vector: Vec<f32> = salarios_all.clone();

                sort_vector.sort_by(|a, b| b.partial_cmp(a).unwrap());

                println!("\nOrdenación Decreciente");

                show_salarios(sort_vector);

                let salario_min: f32 = get_salario_min(salarios_all.clone());
                let salario_max: f32 = get_salario_max(salarios_all.clone());

                println!("\nSalario Min:\t{}  €\tSalario Max:\t{}  €\n", salario_min, salario_max);
                
                let salario_medio: f32 = get_media_salarios(salarios_all).clone();

                println!("\nSalario Medio:\t{}  €\n", salario_medio);

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

/* REVIAR LAS CONDICIONES  CON UN VALOR INTERMEDIO PARA VALORAR EL VALOR ANTERIOR EN RESULT  Y CON PARARLO CON EL ACTUAL Y EL POSTERIOR SI ES POSIBLE   */
fn get_salario_min(salarios_all: Vec<f32>) -> f32 {
    let mut result: f32 = 0.0;
    let mut aux_value: f32;
    let mut aux_ind: usize = 1;

        for i in salarios_all.iter() {
            aux_value = *i;
            // let aux_ind :usize = ((i / 1.0) as i32 + 1) as usize;

                if aux_ind < salarios_all.len() && aux_value < salarios_all[aux_ind] {
                    result = aux_value;

                } else {
                    
                    if aux_ind < salarios_all.len() {
                        result = salarios_all[aux_ind];
                    }

                }

            aux_ind += 1;    
        }

    return result;
}

/* REVIAR LAS CONDICIONES  CON UN VALOR INTERMEDIO PARA VALORAR EL VALOR ANTERIOR EN RESULT  Y CON PARARLO CON EL ACTUAL Y EL POSTERIOR SI ES POSIBLE   */
fn get_salario_max(salarios_all: Vec<f32>) -> f32 {
    let mut result: f32 = 0.0;
    let mut aux_value: f32;
    let mut aux_ind: usize = 1;

        for i in salarios_all.iter() {
            aux_value = *i;
            // let aux_ind :usize = ((i / 1.0) as i32 + 1) as usize;

                if aux_ind < salarios_all.len() && (aux_value > salarios_all[aux_ind]) {
                    result = aux_value;

                } else {

                    if aux_ind < salarios_all.len() {
                        result = salarios_all[aux_ind];
                    }

                }

            aux_ind += 1;
        }

    return result;
}

fn get_media_salarios(salarios_all: Vec<f32>) -> f32 {
    let result: f32;

    let mut sumatorio_sal: f32 = 0.0;
    let leng_vector: f32 = salarios_all.len() as f32;
        
        for ind in salarios_all.iter() {
            sumatorio_sal += *ind;
        }

        result = sumatorio_sal / leng_vector; 

    return result;
}

fn show_salarios(salarios_all: Vec<f32>) {
    println!("\nSalarios Totales");

        for value in salarios_all.iter() {
            println!("\n{}\t€", *value);
        }
    println!("");
}