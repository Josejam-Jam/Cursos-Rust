/*  Ejer 14     Crear una aplicación que solicite al usuario una fecha (dia, mes y año) y muestre la fecha
                correspondiente al día siguiente. 
*/

use std::io;
use std::str::FromStr;

fn main() {

    let mut dias: i32;
    let mut mes: i32;
    let mut year: i32;

    let result_data: (i32, i32, i32);

        loop {

            println!("\nIntroduce Día del Mes");
            let mut entrada1 = String::new();

            io::stdin().read_line(&mut entrada1).expect("Error en la Lectura de datos");
            dias = i32::from_str(&entrada1.trim()).unwrap();

                if dias >= 1 && dias <= 31 {
                    break;
                } else {
                    println!("\nError el Día Introducido es erróneo\n");
                }
        }

        loop {

            println!("\nIntroduce Mes del Año");
            let mut entrada2 = String::new();

            io::stdin().read_line(&mut entrada2).expect("Error en la Lectura de datos");
            mes = i32::from_str(&entrada2.trim()).unwrap();

                if mes >= 1 && mes <= 12 {
                    break;
                } else {
                    println!("\nError el Mes Introducido es erróneo\n");
                }
        }

        loop {
            println!("\nIntroduce Año");
            let mut entrada3 = String::new();

            io::stdin().read_line(&mut entrada3).expect("Error en la Lectura de datos");
            year = i32::from_str(&entrada3.trim()).unwrap();

                if year >= 0 {
                    break;
                } else {
                    println!("\nError el Año Introducido es erróneo\n");
                }
        }

    result_data = get_increment_dia_tupla(dias, mes, year);

    get_print_fecha_actual(result_data);
}

fn get_dias_mes(mes: i32) -> i32 {
    let mut result_dias: i32 = 0;

        if mes == 2 {
            result_dias = 28;
        }

        if mes == 4 || mes == 6 || mes == 9 || mes == 11  {
            result_dias = 30
        }

        if mes == 1 || mes == 3 || mes == 5 || mes == 7 || mes == 8 || mes == 10 || mes == 12 {
            result_dias = 31;
        }

    return result_dias;
}

fn get_increment_dia_tupla(mut dias: i32, mut mes: i32, mut year: i32) -> (i32, i32, i32) {

    let dias_mes: i32 = get_dias_mes(mes);

    dias += 1;

        if dias > dias_mes {
            dias = 1;

            mes += 1; 
        }

        if mes > 12 {
            year += 1;

            mes = 1;
        }

        if year == 0 {
            year = 1;
        }

    let result_tupla = (dias, mes, year);

    return result_tupla;

}

fn get_print_fecha_actual(result_tupla: (i32, i32, i32)) {
    let aux_dias;
    let aux_mes;
    let aux_year;

        if result_tupla.0 < 10 {
            aux_dias = "0".to_string() + &result_tupla.0.to_string();
        } else {
            aux_dias = result_tupla.0.to_string();
        }

        if result_tupla.1 < 10 {
            aux_mes = "0".to_string() + &result_tupla.1.to_string();
        } else {
            aux_mes = result_tupla.1.to_string();
        }

        if result_tupla.2 < 10 {
            aux_year = "000".to_string() + &result_tupla.2.to_string();

        } else if result_tupla.2 >= 10 && result_tupla.2 < 100 {
            aux_year = "00".to_string() + &result_tupla.2.to_string();

        } else if result_tupla.2 >= 100 && result_tupla.2 < 1000 {
            aux_year = "0".to_string() + &result_tupla.2.to_string();

        }  else {
            aux_year = result_tupla.2.to_string();
        }     
    
    println!("\nHora :\t{}-{}-{}\n", aux_dias, aux_mes, aux_year);
}

