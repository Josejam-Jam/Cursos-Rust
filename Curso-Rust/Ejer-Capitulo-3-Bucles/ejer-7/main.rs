/*      Ejer    7       Diseñar un programa que muestre el producto de los 10 primeros números impares.      */

fn main() {
    const NUM_ITERACIONES: i32 = 10;
    const NUM_INCREMENTO: i32 = 2;

    println!("\nMostamos el Producto de los 10 Primeros Números Impares\n");

    show_product_odd(NUM_ITERACIONES, NUM_INCREMENTO);

}

fn get_product_numbers(result_acum: i32, num: i32) -> i32 {
    return result_acum * num;
}

fn show_product_odd(num_iteraciones: i32, increment: i32) {
    let mut result_product;
    let mut aux_num = 1;
    let mut aux_ind = 1;

    result_product = aux_num;

        loop {

            if aux_ind == num_iteraciones {
                println!("[ ( n + 2 ) * ( n + 2 ) ] \u{207F}\t=\t{}\n", result_product);
                break;
            } else {
                result_product = get_product_numbers(result_product, aux_num);

                aux_num += increment;
            }
            aux_ind += 1;
        }
}
