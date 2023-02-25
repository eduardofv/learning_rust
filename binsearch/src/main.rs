use rand::Rng;


fn gen_array(size: usize, max_int:u32) -> Vec<u32> {
    let mut arr = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..size {
        let n: u32 = rng.gen_range(0..max_int);
        arr.push(n);
    }
    arr.sort();
    arr
}


fn binsearch_rec(arr: &Vec<u32>, value: u32, offset: isize) -> isize {
    println!("---");
    dbg!(&arr);
    dbg!(offset);
    
    if arr.is_empty() {
        return -1
    }

   if arr.len() == 1 {
        if arr[0] == value {
            return offset
        } else {
            return -1
        }
   }

   let mid = arr.len() / 2;
   dbg!(mid);
   dbg!(arr[mid]);

   if arr[mid] == value {
       return offset + mid as isize
   }

   if value < arr[mid] {
       return binsearch_rec(&arr[..mid].to_vec(), value, offset)
   } else {
       return binsearch_rec(&arr[mid..].to_vec(), value, offset + mid as isize)
   }
}

fn main() {
    let arr = gen_array(10, 1000);

    /*
    let target = rand::thread_rng().gen_range(0..arr.len());
    let value = arr[target];
    */

    let target = -1;
    let value = 1001;

    println!("looking for value: {}", value);
    println!("looking for index: {}", target);
    let ret = binsearch_rec(&arr, value, 0);
    if ret == -1 {
        println!(">>> not found");
    } else {
        println!(">>> found index: {}", ret);
        println!(">>> found value: {}", &arr[ret as usize]);
    }

    if target as isize != ret as isize {
        print!("NO ");
    }
    println!("OK")
}
