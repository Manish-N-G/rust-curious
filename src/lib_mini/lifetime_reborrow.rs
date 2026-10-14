fn normal_move() -> i32 {
    let mut a = 8;
    let b:&mut i32 = &mut a;
    *b += 1;
    let c = b;
    *c += 1;

    // *b += 1; Error this doesnt compiles
    println!("a is {}", a);
    a
}

fn reborrow() -> i32 {
    let mut a = 8;
    let b: &mut i32 = &mut a;
    *b += 1;
    let c: &mut i32 = b;
    *c += 1;

    *b += 1;  // Does compile because of reborrow
    println!("a is {}", a);
    a
}

fn reborrow_implicit() -> i32 {
    let mut a = 8;
    let b: &mut i32 = &mut a;
    *b += 1;
    let c: &mut _ = b; // reborrowed and compiler implies value
    *c += 1; 

    *b += 1;  // Does compile because of reborrow
    println!("a is {}", a);
    a
}

fn cast_reborrow() -> i32 {
    let mut a = 8;
    let b: &mut i32 = &mut a;
    *b += 1;
    // let c = b; // gives error when *b += 1;
    let c = b as &mut _;  // reborrowed and cast implies value
    *c += 1; 

    *b += 1;  // Does compile because of reborrow
    println!("a is {}", a);
    a
}

fn reborrow_explicit() -> i32 {
    let mut a = 8;
    let b: &mut i32 = &mut a;
    *b += 1;
    let c = &mut *b; // explicit reborrow for you time
    *c += 1; 

    *b += 1;  // Does compile because of reborrow
    println!("a is {}", a);
    a
}

#[allow(clippy::needless_lifetimes)]
// technically this is val: &'x mut *i32 g
fn reborrow_value<'x>(val: &'x mut i32) -> &'x mut i32 {
    val
}

fn reborrow_with_fn() -> i32 {
    let mut a = 8;
    let b:&mut i32 = reborrow_value(&mut a);
    *b += 1;
    let c = b;
    *c += 1;

    // *b += 1; // does not compile
    println!("a is {}", a);
    a
}


fn reborrow_lifetime_val() -> i32 {
    let mut a = 8;
    let b:&mut i32 = reborrow_value(&mut a);
    *b += 1;
    let c = reborrow_value(b); // value reborrowed here
    *c += 1;

    *b += 1; // This does compile
    println!("a is {}", a);
    a
}


#[test]
fn copy_borrow() {
    assert_eq!(normal_move(), 10);
    assert_eq!(reborrow_with_fn(), 10);
}


#[test]
fn reborrow_borrow() {
    assert_eq!(reborrow(), 11);
    assert_eq!(reborrow_lifetime_val(), 11);
}

#[test]
fn cast_explicit_implicit_reborrow() {
    assert_eq!(reborrow_implicit(), 11);
    assert_eq!(cast_reborrow(), 11);
    assert_eq!(reborrow_explicit(), 11);
}
