#[derive(Debug)]
struct CustSmartPtr {
    data: String,
}

impl Drop for CustSmartPtr {
    fn drop(&mut self) {
        println!("Dropping {}", self.data);
    }
}

fn main() {
    let mut c = CustSmartPtr {
        data: String::from("hola")
    };

    println!("C is {:?}", c);

    {
        println!("in block");

        let mut d = c;
        d.data = String::from("changed");
        println!("d is {:?}", d);

        println!("ending block");
    }
    
    println!("finishing");
    //println!("{}", c.data);
}
