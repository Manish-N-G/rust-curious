fn main() {
    // borrowing with other concepts
    // reborrowing is when we have *b of refernce that allows
    // us to point back to a type.

    // ================ reborrowing =======================
    let mut x = 33;
    let y:&mut i32 = &mut x;
    let z:&&mut i32 = &y;
    let a:&i32 = &*y; // but this compiles?
    // let b:&mut i32 = &mut *y; // wont compile
    // NOTE: This compiles because of NLL (non-lexical lifetimes)
    println!("val is {} {} {}", /*x*/ y, z, a);


    let mut x = vec![1,3,4];
    let y = &mut x;
    // println!("{:?}, {:?}", x,y);
    // println!("{:?}, {:?}", x,y);
    let z = &y;
    println!("{:?}, {:?}", y,z);
    // let z = &x;
    // println!("{:?}, {:?}", y,z);
    let z = &*y;
    println!("{:?}, {:?}", y, z);
    y.push(1);


    let mut a = 10;
    let b = &mut a;
    let _ = &*b;
    let c:&i32 = b; // same as above
    println!("{} {}", b, c);
    a+=1;
    println!("{}", a);


    let mut a = String::from("hello");
    a.push_str(" there");
    let b = &a;
    println!("{:?} {:?}", a, b);
    // let b = &mut a; // wont compile for print
    // println!("{:?} {:?}", a, b); // wont compile for print
    let b = a.as_bytes();
    println!("{:?} {:?}", a, b);
    a.push_str(" man");
    
}
