/*      Ejer    11      Diseñar una aplicación que muestre las tablas de multiplicar del 1 al 10.

*/

fn main() {
    const INIT_NUM: i32 = 1;
    const MAX_NUM: i32 = 10;
    let mut iterador: i32 = 0;

        loop {

            let aux_num = INIT_NUM + iterador;

            println!("Tabla de Multiplicar Número\t{}\n", aux_num);

            show_tabla_multiplicar(aux_num);

            iterador += 1;

            if iterador >= MAX_NUM {  break;  }

        }
}


fn get_multiplicar(num: i32, multiplicando: i32) -> i32 {
    return num * multiplicando;
}

fn show_tabla_multiplicar(num: i32) {
    let mut aux_mult: i32 = 0;

        loop {

            if aux_mult > 10 {  break; }

            if aux_mult <= 10 {
                println!("\t{}\tx\t{}\t=\t{}\n",num, aux_mult, get_multiplicar(num, aux_mult));
            }

            aux_mult += 1;
        }

}
