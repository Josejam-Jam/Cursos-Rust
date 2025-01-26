/*  Ejer 13     Escribir un programa que pida un hora de la forma: hora, minutos y segundos
                El programa debe mostrar la hora un segundo más tarde

                Un ejemplo:

                hora actual [10:41:59] hora actual + 1 segundo [10:42:00]
*/

use std::io;
use std::str::FromStr;

fn main() {

    let mut hora: i32;
    let mut min: i32;
    let mut seg: i32;
    let result_data: (i32, i32, i32);

        loop {

            println!("\nIntroduce Hora Formato [ 24H ]");
            let mut entrada1 = String::new();

            io::stdin().read_line(&mut entrada1).expect("Error en la Lectura de datos");
            hora = i32::from_str(&entrada1.trim()).unwrap();

                if hora >= 1 && hora <= 24 {
                    break;
                } else {
                    println!("\nError la Hora introducida es errónea\n");
                }
        }

        loop {

            println!("\nIntroduce Minutos");
            let mut entrada2 = String::new();

            io::stdin().read_line(&mut entrada2).expect("Error en la Lectura de datos");
            min = i32::from_str(&entrada2.trim()).unwrap();

                if min >= 0 && min <= 59 {
                    break;
                } else {
                    println!("\nError la Minutos introducida es errónea\n");
                }
        }

        loop {
            println!("\nIntroduce Segundos");
            let mut entrada3 = String::new();

            io::stdin().read_line(&mut entrada3).expect("Error en la Lectura de datos");
            seg = i32::from_str(&entrada3.trim()).unwrap();

                if seg >= 0 && seg <= 59 {
                    break;
                } else {
                    println!("\nError la Hora introducida es errónea\n");
                }
        }
    
        
    result_data = get_increment_hora_tupla(hora, min, seg);
    // println!("{}", result_data.to_string());

    print_incrementar_hora_tupla(result_data);

    //get_print_increment_hora(hora, min, seg);
}

fn get_increment_hora_tupla(mut hora: i32, mut min: i32, mut seg: i32) -> (i32, i32, i32) {

        if seg + 1 == 60 {
            min += 1;
            seg = 0;
        } 
        
        if min == 60 {
            hora += 1;
            min = 0;

        }
        
        if hora > 23 {
            hora = 0;
        } 

    let result_tupla = (hora, min, seg);

    return result_tupla;
}

fn print_incrementar_hora_tupla(result_tupla: (i32, i32, i32)) {
    // let result: (String, String, String);
    let aux_hora;
    let aux_min;
    let aux_seg;

        if result_tupla.0 == 0 || result_tupla.0 < 10 {
            aux_hora = "0".to_string() + &result_tupla.0.to_string();
        } else {
            aux_hora = result_tupla.0.to_string();
        }

        if result_tupla.1 == 0 || result_tupla.1 < 10 {
            aux_min = "0".to_string() + &result_tupla.1.to_string();
        } else {
            aux_min = result_tupla.1.to_string();
        }

        if result_tupla.2 == 0 || result_tupla.2 < 10 {
            aux_seg = "0".to_string() + &result_tupla.2.to_string();
        } else {
            aux_seg = result_tupla.2.to_string();
        }     
    
    println!("\nHora :\t{}:{}:{}\n", aux_hora, aux_min, aux_seg);
    
    /* 
    result = (hora, min,  seg);

    println!("\nHora :\t{}:{}:{}\n", result.0, result.1, result.2);
    */    
}
/*
fn get_print_increment_hora(mut hora: i32, mut min: i32, mut seg: i32) {
    
        if seg + 1 == 60 {
            min += 1;
            seg = 0;
        } 
        
        if min == 60 {
            hora += 1;
            min = 0;

        }
        
        if hora > 23 {
            hora = 0;
        }

    let result_tupla = (hora, min, seg);

    print_incrementar_hora_tupla(result_tupla);

}
*/