/*      Ejer 2      Un centro educativo no ha pedido que diseñemos una aplicación para calcular algunos datos
                    estadísticos de las edades de los alumnos.

                    Se introducirán datos hasta que uno de ellos sea negativo. La aplicación mostrará la suma de
                    todas las edades, la media, de cuántos alumnos hemos introducido las edades y cuántos
                    alumnos son mayores de edad. Implementar la aplicación requerida.
 
 */

use std::io;
use std::str::FromStr;
use regex::Regex;

fn main() {
    let mut edad:i32;
    let mut suma_edades: i32 = 0;
    let mut media_edades: f32 = 0.0;
    let mut cont_alumnos: i32 = 0;
    let mut cont_mayores_edad: i32 = 0;
    let exp_regex = Regex::new(r"-?\d+").unwrap();

        loop {

            println!("\nIntroduce la edad del alumno");
            let mut entrada = String::new();

            io::stdin().read_line(&mut entrada).expect("Error en la lectura de datos");
            edad = i32::from_str(&entrada.trim()).unwrap();

                if exp_regex.is_match(&edad.to_string()) {

                    if edad > 0 {
                        cont_alumnos = get_cant_alumnos(cont_alumnos);

                            if is_mayor_edad(edad) {
                                cont_mayores_edad += 1;
                            }

                        suma_edades = get_sum_edades(suma_edades, edad);
                        media_edades = get_media_edades(suma_edades, cont_alumnos);
                    }
                    
                    
                    if edad < 0 {
                        break;
                    }

                }

        }

    println!("\nSumatorio Edades:\t{}\nMedia Edad:\t{}  Años\nCantidad Alumnos:\t{}\nCantidad Alumnos Mayores de Edad:\t{}\n", suma_edades, media_edades, cont_alumnos, cont_mayores_edad);
}

fn is_mayor_edad(edad: i32) -> bool {
    return edad >= 18;
}

fn get_cant_alumnos(cont_alumnos: i32) -> i32 {
    let result = cont_alumnos + 1;

    return result; 
}

fn get_sum_edades(sumatorio_edades: i32, edad: i32) -> i32 {
    return sumatorio_edades + edad;
}

fn get_media_edades(sumatorio_edades: i32, cont_alumnos: i32) -> f32 {
    return sumatorio_edades as f32 / cont_alumnos as f32;
}
