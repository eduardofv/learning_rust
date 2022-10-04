struct CustSmartPtr {
    data: String,
}

impl Drop for CustSmartPtr {
    fn drop(&mut self) {
        println!("Dropping {}", self.data);
    }
}

fn main() {
    let c = CustSmartPtr {
        data: String::from("hola")
    };


    let d = CustSmartPtr {
        data: String::from("adios")
    };

    println!("Created...");
    drop(c);
    println!("c dropped");
    //println!("{}", c.data);
}
