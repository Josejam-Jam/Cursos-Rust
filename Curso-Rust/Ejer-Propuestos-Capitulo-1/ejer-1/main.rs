/*  Ejer 1  Un economista nos ha encargado un programa para realizar cálculos con el IVA, la aplicación
            debe solicitar la base imponible y el IVA a aplicar, debemos mostrar en pantalla el importe
            correspondiente al IVA y el Total.
*/

use std::io;
use std::str::FromStr;

fn main() {

    const SUPER_REDUCIDO :f32 = 0.04;
    const REDUCIDO: f32 = 0.10;
    const GENERAL: f32 = 0.21;

    let base_imponible: f32;
    let mut tipo_iva: f32 = 0.0;
    let iva: f32;
    let precio_total: f32;

    println!("\nIntroduce Base Imponible");

    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Error en la lectura de datos");

    base_imponible = f32::from_str(&entrada.trim()).unwrap();

    //let data_ok = false;
    let mut opcion;

        loop {

            println!("\n\tMenú Tipo IVA\n");
            println!("Opción 1:\tTipo Super Reducido 4%\n");
            println!("Opción 2:\tTipo Reducido 10%\n");
            println!("Opción 3:\tTipo General 21 4%\n");

            println!("\nIntroduce Opción [ 1 - 3 ]");
            
            let mut entrada2 = String::new();
            io::stdin().read_line(&mut entrada2).expect("Error en la lectura de datos");

            opcion = u32::from_str(&entrada2.trim()).unwrap();

            if opcion == 1 || opcion == 2 || opcion == 3 {
                break;
            }
        }

        if opcion == 1 {  tipo_iva = SUPER_REDUCIDO;  }

        if opcion == 2 {  tipo_iva = REDUCIDO;  }

        if opcion == 3 {  tipo_iva = GENERAL;  }


    iva = get_calc_iva(base_imponible, tipo_iva);

    precio_total = get_calc_precio_total_iva(base_imponible, iva);

    println!("\nBase Imponible:\t{}\t€", base_imponible);
    println!("\nTotal IVA:\t{}\t€", iva);
    println!("\nImporte Total:\t{}\t€\n", precio_total);


}

fn get_calc_iva(precio: f32, tipo_iva: f32) -> f32 {
    return precio * tipo_iva;
}

/* 
fn get_calc_precio_total_tipoiva(precio: f32, tipo_iva: f32) -> f32 {
    return precio * (tipo_iva + 1.0);
}
*/

fn get_calc_precio_total_iva(precio: f32, iva: f32) -> f32 {
    return precio + iva;
}
