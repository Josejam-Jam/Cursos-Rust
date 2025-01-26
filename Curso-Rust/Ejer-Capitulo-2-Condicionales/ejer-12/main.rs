/*  Ejer 12     Pedir el día mes y año de una fecha e indicar si la fecha es correcta. Recordamos que existen
                meses con 28, 30 y 31 días. 
                
                No consideramos los años bisiestos.
*/

use std::io;
use std::str::FromStr;

fn main() {
    let mut dia: i32;
    let mut mes: i32;
    let mut year: i32;

    // let mut fecha_correcta: bool = false;
    let fecha_correcta: bool;

        loop {

            println!("\nIntroduce Día del Mes rango [ 1 - 31 ]");
            let mut entrada1 = String::new();
        
            io::stdin().read_line(&mut entrada1).expect("Error en la lectura");
            dia = i32::from_str(&entrada1.trim()).unwrap();

                if dia >= 1 && dia <= 31 {
                    break;
                } else {
                    println!("\nError dia Fuera de Rango");
                }
        }

        loop {

            println!("\nIntroduce Número de Mes rango [ 1 - 12 ]");
            let mut entrada2 = String::new();
        
            io::stdin().read_line(&mut entrada2).expect("Error en la lectura");
            mes = i32::from_str(&entrada2.trim()).unwrap();
        

                if mes >= 1 && mes <= 12 {
                    break;
                } else {
                    println!("\nError mes Fuera de Rango");
                }
        }

        loop {

            println!("\nIntroduce Año rango [ 0 - x ]");
            let mut entrada3 = String::new();

            io::stdin().read_line(&mut entrada3).expect("Error en la lectura");
            year = i32::from_str(&entrada3.trim()).unwrap();
                
                if year >= 0 {
                    break;
                } else {
                    println!("\nError dia Fuera de Rango");
                }
        }


        if year == 0 {
            fecha_correcta = false;
        } else {
            /*if mes == 2 {    //mes == 2 && dia >= 1 && dia <= 28
                if  dia >= 1 && dia <= 28 {
                    fecha_correcta = true;
                }

            } else if mes == 4 || mes == 6 || mes == 9 || mes == 11 {   //mes == 4 || mes == 6 || mes == 9 || mes == 11 && dia >= 1 && dia <= 30
                if dia >= 1 && dia <= 30 {
                    fecha_correcta = true;
                }

            } else if mes == 1 || mes == 3 || mes == 5 || mes == 7 || mes == 8 || mes == 10 || mes == 12 {  //mes == 1 || mes == 3 || mes == 5 || mes == 7 || mes == 8 || mes == 10 || mes == 12 && dia >= 1 && dia <= 31
                if  dia >= 1 && dia <= 31 {
                    fecha_correcta = true;
                }
                
            } else {
                fecha_correcta = false;
            }
            */
            if mes == 2 && dia >= 1 && dia <= 28{    
                    fecha_correcta = true;

            } else if mes == 4 || mes == 6 || mes == 9 || mes == 11 && dia >= 1 && dia <= 30 {   
                    fecha_correcta = true;

            } else if mes == 1 || mes == 3 || mes == 5 || mes == 7 || mes == 8 || mes == 10 || mes == 12 && dia >= 1 && dia <= 31 {  
                    fecha_correcta = true;
                
            } else {
                fecha_correcta = false;
            }
        }

        if fecha_correcta {
            println!("\nLa Fecha Introducida \t{}-{}-{}\tEs Correcta\n", dia, mes, year);
        } else {
            println!("\nLa Fecha Introducida \t{}-{}-{}\tNo Es Correcta\n", dia, mes, year);
        }
}
