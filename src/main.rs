/*
Pruebas de tuplas
*/
fn main() {

    // Declara una tupla de tres elementos variados
    let latupla = ( 'E', 5i32, true);

    println!("¿Es la letra {} la {} letra del abecedario? {}", latupla.0, latupla.1, latupla.2 );

    // Ahora una tupla de números
    let otratupla = ( 23, 4.003, 1_000_000_000);

    let (x, y, z) = otratupla;

    println!("Los valores de la otra tupla son: {} {} y {}", x, y, z);
    
}

