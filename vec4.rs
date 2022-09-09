fn main() {
    #[derive(Debug)]
    enum Cell {
        Entero(i32),
        Flotante(f64),
        Cadena(String),
    }

    let x = vec![
        Cell::Entero(19),
        Cell::Cadena(String::from("Hola")),
        Cell::Entero(1),
        Cell::Flotante(1.2),
    ];

    dbg!(x);
}
