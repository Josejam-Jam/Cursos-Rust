/*  Ejer 7  Pedir tres números y mostrarlos ordenados de mayor a menor.   */

use std::io;
use std::str::FromStr;

fn main() {
    let num1: i32;
    let num2: i32;
    let num3: i32;

    /*
    let num1: u32;
    let num2: u32;
    let num3: u32;
    */
    
    println!("\nIntroduce un Número");
    let mut entrada1 = String::new();

    io::stdin().read_line(&mut entrada1).expect("Error en la lectura de datos");
    num1 = i32::from_str(&entrada1.trim()).unwrap();
    // num1 = i32::from_str(&entrada1.trim()).unwrap();

    println!("\nIntroduce un Segundo Número");
    let mut entrada2 = String::new();

    io::stdin().read_line(&mut entrada2).expect("Error en la lectura de datos");
    num2 = i32::from_str(&entrada2.trim()).unwrap();
    // num2 = i32::from_str(&entrada2.trim()).unwrap();

    println!("\nIntroduce un Tercer Número");
    let mut entrada3 = String::new();

    io::stdin().read_line(&mut entrada3).expect("Error en la lectura de datos");
    num3 = i32::from_str(&entrada3.trim()).unwrap();
    // num3 = i32::from_str(&entrada3.trim()).unwrap();

    get_sort_decreciente_i32(num1, num2, num3);

}

fn get_sort_decreciente_i32(num1: i32, num2: i32, num3: i32)  {
    let mut mayor: i32 = 0;
    let mut interm: i32 = 0;
    let mut menor: i32 = 0;

        if num1 > num2 && num1 > num3 {  
            mayor = num1;

                if num2 > num3 { 
                    interm = num2; 
                } else {
                    interm = num3;
                }
                if interm == num2 {
                    menor = num3;
                } else {
                    menor = num2;
                }
        } else if num2 > num1 && num2 > num3 {
            mayor = num2;

                if num1 > num3 {
                    interm = num1;
                } else {
                    interm = num3;
                }

                if interm == num1 {
                    menor = num3;
                } else {
                    menor = num1;
                }

        } else if num3 > num1 && num3 > num2 {
    
            mayor = num3;
            
                if num1 > num2 {
                    interm = num1;
                } else {
                    interm = num2;
                }

                if interm == num1 {
                    menor = num2;
                } else {
                    menor = num1;
                }
        }

    println!("\nMostramos los Número introducidos en Orden Decreciente\t{}\t{}\t{}", mayor, interm, menor);

}
/* fn get_sort_decreciente_u32(num1: u32, num2: u32, num3: u32)  {
    let mut mayor: u32 = 0;
    let mut interm: u32 = 0;
    let mut menor: u32 = 0;

    if num1 > num2 && num1 > num3 {  
            mayor = num1;

                if num2 > num3 { 
                    interm = num2; 
                } else {
                    interm = num3;
                }
                if interm == num2 {
                    menor = num3;
                } else {
                    menor = num2;
                }
        } else if num2 > num1 && num2 > num3 {
            mayor = num2;

                if num1 > num3 {
                    interm = num1;
                } else {
                    interm = num3;
                }

                if interm == num1 {
                    menor = num3;
                } else {
                    menor = num1;
                }

        } else if num3 > num1 && num3 > num2 {
    
            mayor = num3;
            
                if num1 > num2 {
                    interm = num1;
                } else {
                    interm = num2;
                }

                if interm == num1 {
                    menor = num2;
                } else {
                    menor = num1;
                }
        }

    println!("\nMostramos los Número introducidos en Orden Decreciente\t{}\t{}\t{}", mayor, interm, menor);

}
*/    