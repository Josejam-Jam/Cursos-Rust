/*      Ejer    9       Un centro de investigación de la flora urbana necesita una aplicación que muestre cuál es el
                        árbol más alto. 
                        
                        Para ello se introducirá por teclado la altura (en centímetros) de cada árbol
                        (terminando cuando se utilice -1 como altura).

                        Los árboles se identifican mediante etiquetas con números únicos correlativos,
                        comenzando en 0.

                        Se pide diseñar una aplicación que resuelva el problema planteado.

*/

use std::io;
use std::str::FromStr;
use regex::Regex;

fn main() {
    let mut altura:f32;
    let mut num_identify:i32 = 0;
    let mut max_altura:f32 = 0.0;
    let mut result_data: [f32;2] = [0.0, 0.0];
    let mut result_data_vector: Vec<f32> = Vec::new();

    let exp_regex = Regex::new(r"\d+").unwrap();
    let exp_regex_caracter = Regex::new(r"[a-zA-Z]+").unwrap();
    let exp_regex_white_space = Regex::new(r"\s+").unwrap();
    let exp_regex_salida = Regex::new(r"-1").unwrap();

    let mut data_ok: bool;

        loop {

            println!("\nIntroduce la Altura del Árbol [ Decimal ]");
            let mut entrada_1 = String::new();

            io::stdin().read_line(&mut entrada_1).expect("Error en la lectura de datos");

            let aux_ok:&bool = &entrada_1.is_empty();

                if !*aux_ok {

                    if exp_regex_salida.is_match(&entrada_1.trim().to_string()) {
                        /*  ** NOTA EQUIVALE A EXPRESIÓN [ ALTURA == -1 ]

                                if altura == -1 {   break;  }

                        */

                        /*  *** MOSTRAR ALTURA MÁX_ARBOL Y NUM_IDENTIFICATIVO   
                            println!("\nNum Identificador:\t{}\tMax Altura\t{}\n",num_identify, max_altura);
    
                            println!("\nNum Identificador:\t{}\tMax Altura\t{}\n", result_data[0], result_data[1]);                         /*  *** Prueba Array    */  

                            println!("\nNum Identificador:\t{:?}\tMax Altura\t{:?}\n", result_data_vector[0], result_data_vector[1]);       /*  *** Prueba Vector    */  

                            // show_array_max_altura(result_data);      /*  *** Prueba Array    */  
                            // show_vector_max_altura(result_data);     /*  *** Prueba Vector    */  
                        
                        */
                        break;
                    }

                    let aux_data_ok:bool = exp_regex_caracter.is_match(&entrada_1.trim().to_string()) || exp_regex_white_space.is_match(&entrada_1.to_string()) && !exp_regex.is_match(&entrada_1.trim().to_string());
                    // let aux_data_ok:bool = exp_regex_caracter.is_match(&entrada_1.trim().to_string());

                    // if exp_regex_caracter.is_match(&entrada_1.trim().to_string()) {
                        if aux_data_ok {
                            println!("\nError al introducir los Datos Introduzca un número");
                        } 
                        
                        if exp_regex.is_match(&entrada_1.trim().to_string())  {
        
                            altura = f32::from_str(&entrada_1.trim()).unwrap();
        
                            data_ok = exp_regex.is_match(&altura.to_string());
        
                                // if exp_regex.is_match(&altura.to_string()) {
                                if data_ok {

                                    if altura > 0.0 {
                                        //max_altura = get_max_altura(/*num_identify, */altura, max_altura);
                                        result_data = get_max_altura_array(num_identify, altura, max_altura);           /*  *** Prueba Array    */
                                        result_data_vector = get_max_altura_vector(num_identify, altura, max_altura);   /*  *** Prueba Vector    */
                                        num_identify += 1;
                                    }
                                    
                                } else {
                                    println!("\nError al introducir los Datos");
                                }
                        }
                }
        }

    println!("\nNum Identificador:\t{}\tMax Altura\t{}\n",num_identify, max_altura);
    
    println!("\nNum Identificador:\t{}\tMax Altura\t{}\n", result_data[0], result_data[1]);                         /*  *** Prueba Array    */  

    println!("\nNum Identificador:\t{:?}\tMax Altura\t{:?}\n", result_data_vector[0], result_data_vector[1]);       /*  *** Prueba Vector    */  

    // show_array_max_altura(result_data);      /*  *** Prueba Array    */  
    // show_vector_max_altura(result_data);     /*  *** Prueba Vector    */  
    
}

/* fn get_max_altura(/*mut num_identify: i32, */altura: f32, max_altura: f32) -> f32 {
    let mut result:f32 = altura;

        if max_altura > altura {
            result = max_altura;
            // num_identify -= 1;
        }

    return result;
} 
*/

fn get_max_altura_vector(num_identify: i32, altura: f32, max_altura: f32) -> Vec<f32> {     /*  *** Prueba Vector    */
    let result:f32 = altura;
    let mut aux_num_identify = num_identify as f32;
    let result_vector: Vec<f32> = Vec::new();

        if max_altura > altura {
            result = max_altura;
            aux_num_identify -= 1.0;
        }
    
    result_vector.push(result);
    result_vector.push(aux_num_identify);

    return result_vector;
} 

fn get_max_altura_array(num_identify: i32, altura: f32, max_altura: f32) ->  [f32; 2] {     /*  *** Prueba Array    */
    let mut result:f32 = altura;
    let mut aux_num_identify = num_identify as f32;
    let result_array: [f32; 2];

        if max_altura > altura {
            result = max_altura;
            aux_num_identify -= 1.0;

        }

    result_array = [aux_num_identify, result];

    return result_array;
}

/* fn show_array_max_altura(result_data: [f32; 2]) {
    
    println!("\nNum Identificador:\t{}\tMax Altura\t{}\n", result_data[0], result_data[1]);
}
*/

/* fn show_vector_max_altura(result_data: Vec<f32>) {
    
    println!("\nNum Identificador:\t{:?}\tMax Altura\t{:?}\n", result_data[0], result_data[1]);
}
*/ 