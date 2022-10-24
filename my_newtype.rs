use std::fmt;
use std::ops::Deref;

trait Reversible {
    type Output;

    fn rev(&self) -> Self::Output;
}


impl Reversible for String {
    type Output = String;

    fn rev(&self) -> String {
        self.chars().rev().collect::<String>()
    }
}

//newtype
struct NewString(String);

impl fmt::Display for NewString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.rev())
    }
}

impl Deref for NewString {
    type Target = String; 

    fn deref(&self) -> &String {
        &(self.0)
    }
}

fn main() {
    let s = String::from("Hola");

    println!("{}", s.rev());

    let ns = NewString(String::from("Adios"));

    println!("{ns}");
    //esto se puede gracias a la implementacion de Deref
    println!("{:?}", ns.chars());

    let ns2 = NewString(String::from("Nuevo String"));
    println!("{ns2}");
    println!("{:?}", ns2.chars());
}

