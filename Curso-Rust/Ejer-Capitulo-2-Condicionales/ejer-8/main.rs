/*  Ejer 8  Pedir los coeficientes de una ecuación de 2º Grado, y mostrar sus soluciones reales

            Si no existen, debe indicarlo

            Recordemos que las soluciones de una ecuación de segundo grado.

                ax² + bx + c = 0 son
                
                x = ( -b (+-) raíz(b² - 4ac) ) / 2a
*/

use std::io;
use std::str::FromStr;

fn main() {
    let a: f32;
    let b: f32;
    let c: f32;

    let x1: f32;
    let x2: f32;
    let discrim: f32;

    println!("\nIntroduzca el Primer Coeficiente ( a )");
    let mut entrada1 = String::new();

    io::stdin().read_line(&mut entrada1).expect("Error en la lectura de datos");
    a = f32::from_str(&entrada1.trim()).unwrap();

    println!("\nIntroduzca el Segungo Coeficiente ( b )");
    let mut entrada2 = String::new();

    io::stdin().read_line(&mut entrada2).expect("Error en la lectura de datos");
    b = f32::from_str(&entrada2.trim()).unwrap();

    println!("\nIntroduzca el Tercer Coeficiente ( c )");
    let mut entrada3 = String::new();

    io::stdin().read_line(&mut entrada3).expect("Error en la lectura de datos");
    c = f32::from_str(&entrada3.trim()).unwrap();

    discrim = get_discriminante(a, b, c);

        if discrim < 0.0 {
            println!("\nNo existen Soluciones Reales\n");
        } else {
            if a == 0.0 {
                println!("\nNo es una ecuación de Segundo Grado\n");
            } else {
                x1 = get_solucion1(a, b, c, discrim);

                x2 = get_solucion2(a, b, c, discrim);

                println!("\nSolución 1:\t{}\n", x1);

                println!("\nSolución 2:\t{}\n", x2);

            }
        }
}

fn get_discriminante(a: f32, b: f32, c: f32) -> f32 {
    return (b.powf(2.0)) - (4.0 * a * c);
}

fn get_solucion1(a: f32, b: f32, c: f32, _discrim: f32) -> f32 {
    let discrim = get_discriminante(a, b, c).sqrt();

    return (-b + discrim) / (2.0 * a);
}

fn get_solucion2(a: f32, b: f32, c: f32, _discrim: f32) -> f32 {
    let discrim = get_discriminante(a, b, c).sqrt();

    return (-b - discrim) / (2.0 * a);
}
