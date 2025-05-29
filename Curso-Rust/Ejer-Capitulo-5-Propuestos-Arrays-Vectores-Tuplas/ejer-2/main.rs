/*      Ejer    2   Los diseñadores de una aplicación necesitan obtener ordenados los datos de una tabla, pero
                    por restricciones de la aplicación, la tabla debe permanecer inmutable. Una posible solución es
                    hacer una copia de la tabla y ordenarla, manteniendo intacta la tabla original, pero esta
                    alternativa se ha desechado.

                    En su lugar, se ha pensado en crear una segunda tabla donde se almacenan ordenados los
                    índices de la tabla original. Se pide diseñar un algoritmo en el que, dada un tabla, cree otra
                    donde se ordenen mediante los índices la tabla original.

                        Veamos un ejemplo:
                        tabla_original :        [ 3, 5, 1, 4 ]
                        tabla_con_indices :     [ 1, 3, 0, 2 ]


                        Donde tabla_con_indices especifica el lugar que ocupan de forma ordenada los datos de
                        tabla_original.

                    Por ejemplo, el primer elemento de tabla_original, que vale 3, en caso de ordenar los datos,
                    ocupará la posición 2 ( que le corresponde en tabla_con_indice ). En este caso, el i-ésimo
                    elemento de tabla_original ocupará la posición que contiene el i-ésimo elemento de
                    tabla_con_indices.

                    *** NOTA EN ESTE CASO SE IMPLEMENTARÁ UN MODELO DE ENTRADA DE DATOS Y COMO SALIDA A ESTÁ FASE DE ENTRADA
                        SE INTRODUCIRÁ -1 POR TECLADO
*/

            /*  ***     TO DO       
            */

use std::io;
use std::str::FromStr;
use regex::Regex;

fn main() {

    let mut data_original : Vec<i32> = Vec::new();
    let mut data_order_index : Vec<usize> = Vec::new();
    let mut data_num : i32;
                        
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
            println!("\nIntroduce Número");
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

                    let salida_bucle: bool = exp_regex_salida.is_match(&entrada.to_string());        

                        if aux_data_negative && salida_bucle {  break;  }

                        if aux_data_ok || aux_data_negative || aux_data_incomplete {
                            println!("\nError al introducir los Datos");
                        }
                           
                        if !aux_data_ok || ( !aux_data_negative && !aux_data_incomplete ) {

                            if !exp_regex_num_float.is_match(&entrada.to_string()) && !exp_regex_num_float_negative.is_match(&entrada.to_string()) {
                                data_num = i32::from_str(&entrada.trim()).unwrap();

                                data_ok = exp_regex.is_match(&data_num.to_string()) 
                                    || !exp_regex_num_negative.is_match(&entrada.to_string());
            
                                    if data_ok {

                                        data_ok_all.push(true);

                                        data_original.push(data_num);

                                    }
                            }
                        }
                    
                }
        }

        if data_ok_all.len() > 0 && (data_ok_all.len() == data_original.len()) {

            data_ok = is_all_data_ok(data_ok_all);

            if data_ok {

                println!("\nData Original\t\t{:?}\n", data_original);

                let mut sort_vector: Vec<i32> = data_original.clone();

                sort_vector.sort_by(|a, b| a.cmp(b));

                    for value in data_original.iter() {

                        let aux_data = *value;

                        let index = get_index_data(sort_vector.clone(), aux_data);

                        data_order_index.push(index.expect("REASON"));

                    }

                println!("\nData Index Order\t{:?}\n", data_order_index);

                println!("\nMuestra de Datos Extendida");

                show_data_vector_extend(data_original.clone());

                show_data_vector_index_extend(data_order_index.clone());        


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

fn get_index_data(data_vector: Vec<i32>, data: i32) -> Option<usize> {
    let result : Option<usize>;

        result = data_vector.iter().position(|n| *n == data);

    return result;
}

/* SÓLO FUNCIONA SI TIENE DATOS PREVIOS INCLUIDOS EN EL VECTOR  */
/*fn set_data_index_vector(mut data_vector: Vec<usize>, index: Option<usize>)  {
   data_vector.push(index.expect("REASON"));
}*/

fn show_data_vector_extend(data_vector_all: Vec<i32>) {
    println!("\nData Elements");

        for value in data_vector_all.iter() {
            println!(" {}", *value);
        }
        
    println!("");
}

fn show_data_vector_index_extend(data_vector_all: Vec<usize>) {
    println!("\nData Elements");

        for value in data_vector_all.iter() {
            print!(" {}", *value);
        }

    println!("");
}