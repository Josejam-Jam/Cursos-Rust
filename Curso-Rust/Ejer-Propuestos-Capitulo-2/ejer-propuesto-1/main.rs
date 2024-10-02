/*  Ejer-Propuesto 1    Sabiendo que para calcular la letra de un documento nacional de identidad el algoritmo es el
                        
                        siguiente:

                                Obtener el módulo 23 del número del DNI.
                                
                                Según el módulo obtenido y la siguiente tabla, se asigna la letra
                                correspondiente al DNI:

                                    0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22
                                    T R W A G M Y F P D  X  B  N  J  Z  S  Q  V  H  L  C  K  E

                        Diseñar una aplicación en la que, dado un número de DNI, calcule la letra que le corresponde.

*/

use std::io;
use std::str::FromStr;
use regex::Regex;
// use regex;

fn main() {

    let vector_letras_dni = vec!['T','R','W','A','G','M','Y','F','P','D','X','B','N','J','Z','S','Q','V','H','L','C','K','E'];
    //let vector_string_letras_dni: Vec<&str> = vec!("T","R","W","A","G","M","Y","F","P","D","X","B","N","J","Z","S","Q","V","H","L","C","K","E");

    //let array_letras_dni: [char; 23] = ['T', 'R', 'W', 'A', 'G', 'M', 'Y', 'F', 'P', 'D', 'X', 'B', 'N', 'J', 'Z', 'S', 'Q', 'V', 'H', 'L', 'C', 'K', 'E'];
    //let array_string_letras_dni: [&str; 23] = ["T" ,"R", "W", "A", "G", "M", "Y", "F", "P", "D", "X", "B", "N", "J", "Z", "S", "Q", "V", "H", "L", "C", "K", "E"];
    
    //let tupla_letras_dni = ('T','R','W','A','G','M','Y','F','P','D','X','B','N','J','Z','S','Q','V','H','L','C','K','E');

    // let num_dni: i32 = 0;
    
    // let re = Regex::new(r"[0-9]{8}").unwrap();
    let exp_regex = Regex::new(r"[\d]{8}").unwrap();

    let mut num_dni: u32;
    // let num_dni: u8;

    // let ind: u8;
    // let ind: unsize;

        loop {

            println!("\nIntroduce Número de DNI [ 8 Dígitos ]\n");
            let mut entrada = String::new();

            io::stdin().read_line(&mut entrada).expect("Error en la lectura de datos");
            num_dni = u32::from_str(&entrada.trim()).unwrap(); 

            // let aux: &str = &num_dni.to_string() as &str;

                if exp_regex.is_match(&num_dni.to_string()) {
                    break;
                } else {
                    println!("\nError en la Introducción de Datos\n");
                }
        }

    // let  ind = get_index_letra_dni_u8(num_dni);
    let ind = get_index_letra_dni_unsize(num_dni);
    
    // let mut salida_letra = String::new();

    let salida_letra: String = vector_letras_dni[ind].to_string();
    // salida_letra = array_letras_dni[ind].to_string();  
    // salida_letra = tupla_letras_dni.ind.to_string(); 
           
            
    /*  MUESTRA LA SALIDA DE LA FUNCIÓN DIRECTAMENTE EN EL PRINTLN! */
    // println!("\nFunctions:\t{}\n", get_letra_dni_vector(ind, vector_letras_dni));

    /*  MUESTRA LA SALIDA EN FORMA LITERAL [ CHAR || STRING ] */
    // println!("\nFunctions Vector String \t{:?}\n", get_letra_dni_vector_string(ind, vector_string_letras_dni));
    // println!("\nFunctions Vector String \t{}\n", get_letra_dni_vector_string(ind, vector_string_letras_dni));


    /*  PRUEBAS   println!("\nArray Prueba\t{:?}\n", array_letras_dni[0]);

    println!("\nTupla Prueba\t{:?}\n", tupla_letras_dni.0);
    */

    /*  VERSIÓN DIRECTA EN EL MAIN  [ FOR  IN ]  */
        /*for i in 0..array_letras_dni.len() + 1 {
            if i == ind.into() { println!("\nAsignación Letra DNI\t{} {}\n", num_dni, array_letras_dni[i]);}
        }
        */

    /*  VERSIÓN DIRECTA EN EL MAIN [ LOOP ] */
        /*loop {
                if iterador == num_dni { 
                    
                    let mut aux: String = String::new();
                    aux = vector_letras_dni[ind].to_string(); 
                    // aux = array_letras_dni[ind].to_string();

                    salida_letra = aux;

                    break;
                } 
                
                if num_dni < iterador {

                    let mut aux: String = String::new();
                    // aux = (vector_letras_dni[0] as str).to_string(); 

                    aux = array_letras_dni[ind].to_string(); 
                    salida_letra = aux;
                    
                    break; 
                }
            iterador += 1;
        }
        
        println!("\nAsignación Letra DNI\t{} {}\n", num_dni, salida_letra);
    */

    /*  VERSIÓN DIRECTA EN EL MAIN [ MATCH ] */
    /*  //let mut aux_ind = 0..23;
        let mut aux_ind = 0..vector_string_letras_dni.len();
        let mut ind: usize = num_dni as usize;
        let mut salida_letra = String::new();

            loop {
                match aux_ind.next() { 

                    Some(aux_ind) => { if aux_ind == ind { salida_letra = vector_string_letras_dni[ind]; }   }, 
                    None => {  salida_letra = "No Imprime nada"; break;     }
                }
            }

        println!("\nAsignación Letra DNI\t{} {}\n", num_dni, salida_letra);        
    */
    

    /*  VERSION A TRAVÉS DE FUNCIÓN PASÁNDOLE UN ARRAY [ CHAR; 23 ]  */    
    // println!("\nAsignación Letra DNI\t{} {}\n", num_dni, get_letra_dni_array(ind, array_letras_dni));

    /*  VERSION DIRECTA EN EL MAIN  [ INDEXACIÓN ] [ ARRAY && VECTORES ] */
    println!("\nAsignación Letra DNI\t{} {}\n", num_dni, salida_letra);
}

/*  VERSIÓN DE EXTRACCIÓN DEL INDICE PARA FUNCIONES TIPADAS CON IND: U8 */
    /* fn get_index_letra_dni_u8(num_dni: u32) -> u8 {
        return (num_dni % 23) as u8;
    }
    */

/*  VERSIÓN DE EXTRACCIÓN DEL INDICE DEVUELVE IND: USIZE QUE PUEDE USARSE COMO INDICE DIRECTO EN [ ARRAYS && VECTORES ]      */
fn get_index_letra_dni_unsize(num_dni: u32) -> usize {
    return (num_dni % 23) as usize;
}


/*  VERSION CON UNA ITERACIÓN [ FOR IN ] SOBRE UN ARRAY [ CHAR; 23 ] DEVOLVIENDO UN STRING CON EL CARÁCTER CORRESPONDIENTE     */
    /* fn get_letra_dni_array(ind: u8, vec_letras_dni: [char; 23]) -> String {
        let mut result: String = String::new();

            for i in 0..vec_letras_dni.len() + 1 {
                // if i == num.into() { println!("\n{}\n", vec_letras_dni[i]);}
                if i == ind.into() { result =  vec_letras_dni[i].to_string();}
            }

        return result;
    }
    */

/*  VERSION CON UNA ITERACIÓN [ FOR IN ] SOBRE UN VECTOR [ VEC<CHAR> ] DEVOLVIENDO UN STRING CON EL CARÁCTER CORRESPONDIENTE       */
    /*fn get_letra_dni_vector(ind: u8, vec_letras_dni: Vec<char>) -> String {
        let mut result: String = String::new();

            for i in 0..vec_letras_dni.len() + 1 {
                // if i == num.into() { println!("\n{}\n", vec_letras_dni[i]);}
                if i == ind.into() { result =  vec_letras_dni[i].to_string();}
            }

        return result;
    }
    */

/*  VERSION CON UNA ITERACIÓN [ FOR IN ] SOBRE UN VECTOR [ VEC<&STR> ] [ STRING LITERALES // PASO REFERENCIA ] DEVOLVIENDO UN STRING CON EL CARÁCTER CORRESPONDIENTE       */
    /*fn get_letra_dni_vector_string(ind: u8, vec_letras_dni: Vec<&str>) -> String {
        let mut result: String = String::new();

            for i in 0..vec_letras_dni.len() + 1 {
                // if i == num.into() { println!("\n{}\n", vec_letras_dni[i]);}
                if i == ind.into() { result =  vec_letras_dni[i].to_string();}
            }

        return result;
    }
    */

/*  VERSION CON UNA ITERACIÓN [ LOOP [ MACH ] ] SOBRE UN VECTOR [ VEC<&STR> ] [ STRING LITERALES // PASO REFERENCIA ] DEVOLVIENDO UN STRING CON EL CARÁCTER CORRESPONDIENTE       */
    /* fn get_letra_dni_match_index_vector_string(ind: u8, vec_letras_dni: Vec<&str>) -> String {
        let mut result = String::new();

        //let mut aux_ind = 0..23;
        let mut aux_ind = 0..vector_string_letras_dni.len();
        let mut ind: usize = num as usize;

            loop {
                match aux_ind.next() { 

                    Some(aux_ind) => { if aux_ind == ind { result = vector_string_letras_dni[ind]; }   }, 
                    None => {  result = "No Imprime nada"; break;     }
                }
            }

        return result;
    }
    */