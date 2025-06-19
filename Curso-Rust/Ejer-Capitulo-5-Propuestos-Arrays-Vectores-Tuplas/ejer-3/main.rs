/*      Ejer    3   Una tabla bidimensional t puede representar un mapa con distintos lugares ( numerados de 0 a n ) 
                    e indicar si existe paso del lugar i al lugar j, mediante el elemento t [ i ][ j ], con un valor true.
                    
                    Diseñar una aplicación que pregunte el número de lugares del mapa, cree una matriz de tipo mapa, 
                    y cargue los pasos que existe entre lugares.


                    *** NOTA EN ESTE CASO SE IMPLEMENTARÁ UN MODELO DE ENTRADA DE DATOS, SOBRE UNA MATRIZ CUADRÁTICA BIDIMENSIONAL,
                        NÚMERO DE FILAS Y COLUMNAS, EL NÚMERO DE FILAS Y COLUMNAS SERÁN IGUALES, GENERANDO,
                        TANTO LOS DATOS COMO LOS VECTORES Y LOS PASOS ENTRE LUGARES DEL MAPA DE FORMA ALEATORIA.

                    Veamos un ejemplo:
                    
                        lugar_0 → lugar_2 → lugar_1 →   lugar_3
                                                |
                                            lugar_0

                    La matriz mapa será:

                        0 false false true  false
                        1 true  false false true
                        2 false true  false false
                        3 false false false false
                            0     1     2     3
                    
                    La aplicación debe solicitar dos lugares, mediante sus números asignados, e indicar si existe
                    algún posible camino entre ellos.

*/

            /*  ***     TO DO       
            */

use std::io;
use std::str::FromStr;
use regex::Regex;
//use rand::Rng;
//use rand::seq::Slice, thread_rng;
use rand::prelude::SliceRandom;

fn main() {

    let mut data_original : Vec<bool> = Vec::new();
    let mut data_order_index : Vec<usize> = Vec::new();
    let mut data_filas_colums : i32;
                        
    let exp_regex = Regex::new(r"\d+").unwrap();
    let exp_regex_caracter = Regex::new(r"[a-zA-Z]+").unwrap();
    let exp_regex_white_space = Regex::new(r"\s+").unwrap();
                        
    let exp_regex_num_negative = Regex::new(r"(-\d+)").unwrap();
                        
    /*  *** REGEX PARA LA VERSIÓN DE TIPOS FLOAT      */
    let exp_regex_data_incomplete = Regex::new(r"(\d+)[\,||\.||\']").unwrap();
                        
    let exp_regex_num_float = Regex::new(r"\d+\.\d+").unwrap();
    let exp_regex_num_float_negative = Regex::new(r"-\d+\.\d+").unwrap();
                        
    let mut data_ok: bool;
    let mut data_ok_all: Vec<bool> = Vec::new();

        loop {
            println!("\nIntroduce Número de [ Filas || Columnas ]");
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

                        if aux_data_ok || aux_data_negative || aux_data_incomplete {
                            println!("\nError al introducir los Datos");
                        }
                           
                        if !aux_data_ok || ( !aux_data_negative && !aux_data_incomplete ) {

                            if !exp_regex_num_float.is_match(&entrada.to_string()) && !exp_regex_num_float_negative.is_match(&entrada.to_string()) {
                                data_filas_colums = i32::from_str(&entrada.trim()).unwrap();

                                data_ok = exp_regex.is_match(&data_filas_colums.to_string()) 
                                    || !exp_regex_num_negative.is_match(&entrada.to_string());
            
                                    if data_ok {

                                        data_ok_all.push(true);

                                        // data_original.push(true);

                                        break;

                                    }
                            }
                        }
                    
                }
        }

        if data_ok_all.len() > 0 && data_filas_colums > 0 {

            data_ok = is_all_data_ok(data_ok_all);

            if data_ok {

                let mut data_ind_1: usize;
                let mut data_ind_2: usize;
                //let mut data_ok_all_second: Vec<bool> = Vec::new();

                //println!("\nData Original\t\t{:?}\n", data_original);

                data_original = get_values_true(data_filas_colums, data_original);

                data_original = get_values_false(data_filas_colums, data_original);


                //println!("\nImpresión de datos Original\t{:?}", data_original);

                data_original = get_vector_shuffle(data_original);

                 /* GENERARMOS LA MATRIZ  */

                let data_vector_2d_all: Vec<Vec<bool>> = get_matriz_cuadratica(data_filas_colums, data_original.clone());

                /*  AQUÍ SE DEBE LIMPIAR DE DATOS EL VECTOR DE DATOS DE VALIDACIÓN DE ENTRADA    */
                //let mut data_ok_all: <bool> = Vec::new();
                data_ok_all = Vec::new();

                /*  PROBAR LA FUNCIÓN PERO HASTA AHORA DA UN EXITED O UN WARNING     */
                
                loop {
                        println!("\nIntroduce Número de [ Fila || Lugar 1 ]");
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

                                    if aux_data_ok || aux_data_negative || aux_data_incomplete {
                                        println!("\nError al introducir los Datos");
                                    }
                                    
                                    if !aux_data_ok || ( !aux_data_negative && !aux_data_incomplete ) {

                                        if !exp_regex_num_float.is_match(&entrada.to_string()) && !exp_regex_num_float_negative.is_match(&entrada.to_string()) {
                                            data_ind_1 = usize::from_str(&entrada.trim()).unwrap();

                                            data_ok = exp_regex.is_match(&data_ind_1.to_string()) 
                                                || !exp_regex_num_negative.is_match(&entrada.to_string());
                        
                                                if data_ok {

                                                    data_ok_all.push(true);

                                                    //data_ok_all_second.push(true);

                                                    data_order_index.push(data_ind_1);

                                                    break;

                                                }
                                        }
                                    }
                                
                            }
                    }
                    
                    loop {
                        println!("\nIntroduce Número de [ Columna || Lugar 2 ]");
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

                                    if aux_data_ok || aux_data_negative || aux_data_incomplete {
                                        println!("\nError al introducir los Datos");
                                    }
                                    
                                    if !aux_data_ok || ( !aux_data_negative && !aux_data_incomplete ) {

                                        if !exp_regex_num_float.is_match(&entrada.to_string()) && !exp_regex_num_float_negative.is_match(&entrada.to_string()) {
                                            data_ind_2 = usize::from_str(&entrada.trim()).unwrap();

                                            data_ok = exp_regex.is_match(&data_ind_2.to_string()) 
                                                || !exp_regex_num_negative.is_match(&entrada.to_string());
                        
                                                if data_ok {

                                                    data_ok_all.push(true);

                                                    //data_ok_all_second.push(true);

                                                    data_order_index.push(data_ind_2);


                                                    break;

                                                }
                                        }
                                    }
                                
                            }
                    }


                    if data_ok_all.len() > 0 && data_ok_all.len() == 2 /*data_ok_all_second.len() == 2 */ {

                        data_ok = is_all_data_ok(data_ok_all);
                        
                        //data_ok = is_all_data_ok(data_ok_all_second);

                        if data_ok {

                            //println!("\nImpresión de datos de Nº Vectores\t{:?}", data_original);

                            //println!("\nImpresión de datos Vector Múltiple\t{:?}", data_vector_2d_all);

                            show_data_vector_2d_cuadratica(data_vector_2d_all.clone());

                            show_data_index_vector(data_filas_colums);


                            /*  MOSTRAMOS EL RANGO DE LA INTERSECCIÓN   */

                            show_point_intersection_range(data_order_index.clone());


                            let aux_ind_1 : usize = data_order_index[0] - 1;
                            // let aux_ind_2 : usize = data_order_index[1];

                            /*  COMPROBACIÓN DE LA INTERSECCIÓN DE LOS DOS PUNTOS DEL RANGO  */

                            let result : bool = find_true_indata(data_vector_2d_all[aux_ind_1].clone());

                            println!("\nExiste un camino entre los 2 puntos\t{}\n", result);


                        }

                    }        

                /*  MOSTRAR   */
                /*    
                show_data_vector_2d_cuadratica(data_vector_2d_all);  

                show_data_index_vector(data_vector_2d_all.len());*/       

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

/* fn clean_all_values_data_ok(mut data_ok_all: Vec<bool>)  {
    
    for ind in 0..data_ok_all.len() {
        //print!("{}", data_ok_all[ind]);
        data_ok_all.remove(ind);
        
    }


    /*  COMPROBAR LA SALIDA NONE CON MATCH Y CONSTRUIR LA IMPRESIÓN DEL RESULTADO A PARTIR DE LA EVALUACIÓN   */
    let mut v = vec![1, 2, 3];
    match v.pop() {
        Some(last) => println!("El último elemento es: {}", last),
        None => println!("El vector está vacío"),
    }
    println!("El vector después de pop: {:?}", v);

    return data_ok_all;
}
*/

/*fn clean_all_values_data_ok(mut data_ok_all: Vec<bool>) -> Vec<bool> {
    
    for ind in 0..data_ok_all.len() {
        data_ok_all.remove(ind);
    }

    return data_ok_all;
}
*/

/* fn clean_all_values_data_vector() -> Vec<_> {

}*/

/*
    vector_data_value.iter().find(|value| **value == true);
*/

/*  CUIDADO CON ESTA IMPLEMENTACIÓN QUE EN LOS CASOS EN NO EXISTAN VALORES TRUE, NO SE DEVUELVE BOOLEANO FALSE NI EL VALOR NONE 
    DEVUELVE UN FALLO DE COMPILACIÓN EN LA EJECUCIÓN DE LA FUNCIÓN PERDIENDO REFERENCIA DEL VALOR

    SE ACONSEJA REALIZAR LA ITERACIÓN Y LA BÚSQUEDA EN EL VECTOR DIRECTAMENTE
*/

fn find_true_indata(data_value: Vec<bool>) -> bool {
    
    //return *data_value.iter().find(|value| **value == true).expect("REASON"); OK

    //return *data_value[0][1].iter().find(|value| **value == true).expect("REASON");


    match data_value.iter().find(|value| **value == true) {
        Some(value) => return *value,
        None => return false,
    }

    /*match data_value[0][1].iter().find(|value| **value == true) {
        Some(value) => return *value,
        None => return false,
    }
    */

}


fn get_values_true(data_filas: i32, mut data_vector: Vec<bool>) -> Vec<bool> {
    /*  **VERSIÓN 2 GENERANDO VECTORES INDIVIDUALES DENTRO DE LA FUNCIÓN   */
    //**VERSIÓN 2   let mut vec_result: Vec<bool> = Vec::new();   

        for _ind in 0..data_filas {
            //**VERSIÓN 2   vec_result.push(true);
            data_vector.push(true);
        }

    //**VERSIÓN 2   return vec_result;
    return data_vector;
}

fn get_values_false(data_filas: i32, mut data_vector: Vec<bool>) -> Vec<bool> {
    /*  TIENE QUE CALCULARSE LA CANTIDAD DE VALORES FALSE RESTANDO EL NÉMERO DE FILAS || COLUMNAS A TOTAL DE VALORES DE LA MATRIZ [ FILAS * COLUMNAS ] 
        ES DECIR ( FILAS * COLUMNAS ) - FILAS = DATA_ALL_FALSE
    */
    /*  **VERSIÓN 2 GENERANDO VECTORES INDIVIDUALES DENTRO DE LA FUNCIÓN   */
    //**VERSIÓN 2   let mut vec_result: Vec<bool> = Vec::new();   

    let num_values_false: i32 = (data_filas * data_filas) - data_filas ;

        for _ind in 0..num_values_false {
            //**VERSIÓN 2   vec_result.push(false);
            data_vector.push(false);
        }

    //**VERSIÓN 2   return vec_result;
    return data_vector;
}

fn get_vector_shuffle( mut data_vector: Vec<bool>) -> Vec<bool> {
    // let mut rng = thread_rng();
    let mut rng = rand::rng();

    data_vector.shuffle(&mut rng);

    return data_vector;
}


fn get_matriz_cuadratica(data_filas: i32, data_vector: Vec<bool>) -> Vec<Vec<bool>> {

    /*  GENERARÁ LA MATRIZ A PARTIR DE LOS DATOS INTRODUCIDOS EN LOS VECTORES AÑADIENDO MAYOR ALEATORIEDAD 
        IGUALANDO LA CANTIDAD DE VALORES TRUE && FALSE A LA HORA DE INTRODUCIR LOS DATOS EN EL VECTOR BIDIMENSIONAL, 
        BARAJANDO PREVIAMENTE LOS DATOS DENTRO DEL PROPIO VECTOR USADO PARA ASIGNAR LA ALEATORIDAD EN LA INTRODUCCIÓN DE TRUE||FALSE 

        PARA CALCULAR LA MATRIZ CUADRÁTICA, SE ELEGIRÁN LOS VALORES A INTRODUCIR DE FORMA ALEATORIA YA QUE HABRÁN SIDO 
        INTRODUCIDOS ALEATORIAMENTE Y BARAJADOS, ESE SERÁ EL VECTOR DE VECTORES RESULTANTE.

    */

    let mut result_vector_matriz: Vec<Vec<bool>> = Vec::new();

    let aux_leng_vector: i32 = data_vector.len() as i32;
    let mut ind: i32 = 0;
    let mut aux_ind: usize;

    //println!("\nPrevios inserción Matriz\t{:?}", data_filas);

        loop {

            if ind == aux_leng_vector {
                break;
            }

            if ind < aux_leng_vector {

                let mut aux_vector: Vec<bool> = Vec::new();
                    
                    loop {

                        aux_ind = ind as usize;
                        ind += 1;

                        aux_vector.push(data_vector[aux_ind]);

                            if ind % data_filas == 0 {
                                //result_vector_matriz.push(aux_vector.clone());

                                //println!("\nVector \t{}\t{:?}", ind , aux_vector);

                                break;

                            }
                            
                    }

                    if ind % data_filas == 0 {

                        result_vector_matriz.push(aux_vector);

                    }

            }

        }

    return result_vector_matriz;
}


fn show_data_index_vector(data_filas: i32) {
    for ind in 0..data_filas {
            if ind == 0 {   print!("\t");   }
        print!("\t   {}", ind);
    }

    println!("\n");
}

/*  REVISAR LOS DATOS DE SALIDA EN EL 2º BÚCLE */
fn show_data_vector_2d_cuadratica(data_vector_2d_all: Vec<Vec<bool>>) {
    println!("\nData Elements\n");

        for ind in 0..data_vector_2d_all.len() {
            print!("\t {}", ind);

            for ind2 in data_vector_2d_all[ind].iter() {
                print!("\t {}", *ind2);
            }

            println!("");
        }

    //println!("");
}

fn show_point_intersection_range(data_order_index: Vec<usize>) {
    println!("Rango de intersección\t{:?}", data_order_index);
}
