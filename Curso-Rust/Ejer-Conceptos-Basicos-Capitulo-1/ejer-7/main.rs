/*  Ejer 7  Un frutero necesita calcular los beneficios anuales que obtiene de la venta de manzanas y peras
            Por ese motivo, es necesario diseñar una aplicación que solicite las ventas (en kilos) de cada
            trimestre para cada fruta la aplicación mostrará el importe total sabiendo que el precio del
            kilo de manzanas está fijado en 2.35 € y el kilo de peras está fijado en 1.95 €.  
*/

use std::io;
use std::str::FromStr;

fn main() {
    const PRECIO_MANZANAS: f32 = 2.35;
    const PRECIO_PERAS: f32 = 1.95;

    let mut cant_man: f32;
    let mut cant_peras: f32;
    let mut beneficio_manzanas: f32;
    let mut beneficio_peras: f32;

    let mut beneficios_t1: f32 = 0.0;
    let mut beneficios_t2: f32 = 0.0;
    let mut beneficios_t3: f32 = 0.0;
    let mut beneficios_t4: f32 = 0.0;

    
    
    let mut iterador = 0u32;

        loop {
            iterador += 1;

            println!("\nIntroduce cantidad en Kilos de Manzanas Trimestre {}", iterador);
            
            let mut entrada1 = String::new();
            io::stdin().read_line(&mut entrada1).expect("Error en la lectura de datos");

            cant_man = f32::from_str(&entrada1.trim()).unwrap();

            println!("\nIntroduce cantidad en Kilos de Peras Trimestre {}", iterador);

            let mut entrada2 = String::new();
            io::stdin().read_line(&mut entrada2).expect("Error en la lectura de datos");

            cant_peras = f32::from_str(&entrada2.trim()).unwrap();

                if iterador <= 4 {

                    if iterador == 1 { 
                        beneficio_manzanas = get_beneficios_manzanas(cant_man, PRECIO_MANZANAS);

                        beneficio_peras = get_beneficios_peras(cant_peras, PRECIO_PERAS);

                        beneficios_t1 = get_beneficio_total_trimestral(beneficio_manzanas, beneficio_peras);
                    }

                    if iterador == 2 { 
                        beneficio_manzanas = get_beneficios_manzanas(cant_man, PRECIO_MANZANAS);

                        beneficio_peras = get_beneficios_peras(cant_peras, PRECIO_PERAS);

                        beneficios_t2 = get_beneficio_total_trimestral(beneficio_manzanas, beneficio_peras);
                    }

                    if iterador == 3 {  
                        beneficio_manzanas = get_beneficios_manzanas(cant_man, PRECIO_MANZANAS);

                        beneficio_peras = get_beneficios_peras(cant_peras, PRECIO_PERAS);

                        beneficios_t3 = get_beneficio_total_trimestral(beneficio_manzanas, beneficio_peras);
                    }

                    if iterador == 4 {  
                        beneficio_manzanas = get_beneficios_manzanas(cant_man, PRECIO_MANZANAS);

                        beneficio_peras = get_beneficios_peras(cant_peras, PRECIO_PERAS);

                        beneficios_t4 = get_beneficio_total_trimestral(beneficio_manzanas, beneficio_peras);
                    }
                }

                if iterador == 4 { break; }                

        }

    let beneficios_anuales_totales: f32 = get_beneficio_anual_total(beneficios_t1, beneficios_t2, beneficios_t3, beneficios_t4);

    println!("\nBeneficio Total Anual\t{}\t€\n", beneficios_anuales_totales);    

}

fn get_beneficios_manzanas(cant_man: f32, precio_manzanas: f32) -> f32 {
    return cant_man * precio_manzanas;
}

fn get_beneficios_peras(cant_peras: f32, precio_peras: f32) -> f32 {
    return cant_peras * precio_peras;
}

fn get_beneficio_total_trimestral(beneficio_manzanas: f32, beneficio_peras: f32) -> f32 {
    return beneficio_manzanas + beneficio_peras;
}

fn get_beneficio_anual_total(beneficios_t1: f32, beneficios_t2: f32, beneficios_t3: f32, beneficios_t4: f32) -> f32 {
    return beneficios_t1 + beneficios_t2 + beneficios_t3 + beneficios_t4;
}
