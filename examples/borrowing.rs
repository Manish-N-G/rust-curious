fn main() {
    // borrowing with other concepts
    // reborrowing is when we have *b of refernce that allows
    // us to point back to a type.

    // ================ reborrowing =======================
    let mut x = 33;
    let y: &mut i32 = &mut x;
    let z: &&mut i32 = &y;
    let a: &i32 = &*y; // but this compiles?
    // let b:&mut i32 = &mut *y; // wont compile
    // NOTE: This compiles because of NLL (non-lexical lifetimes)
    println!("val is {} {} {}", /*x*/ y, z, a);
    println!();

    let mut x = vec![1, 3, 4];
    let y = &mut x;
    // println!("{:?}, {:?}", x,y);
    // println!("{:?}, {:?}", x,y);
    let z = &y;
    println!("{:?}, {:?}", y, z);
    // let z = &x;
    // println!("{:?}, {:?}", y,z);
    let z = &*y;
    println!("{:?}, {:?}", y, z);
    y.push(1);
    println!("{:?}", x);
    println!();

    let mut a = 10;
    let b = &mut a;
    let _ = &*b;
    let c: &i32 = b; // same as above
    println!("{} {}", b, c);
    a += 1;
    println!("{}", a);
    println!();

    let mut a = String::from("hello");
    a.push_str(" there");
    let b = &a;
    println!("{:?} {:?}", a, b);
    // let b = &mut a; // wont compile for print
    // println!("{:?} {:?}", a, b); // wont compile for print
    let b = a.as_bytes();
    println!("{:?} {:?}", a, b);
    a.push_str(" man");
    println!("{}", a);
    println!();

    // for structs and tupes, the compiler is a bit more smarter.
    // it can handle the individual elements without throwing an
    // error for borrow and borrow mut.
    // ===================== partial borrowing ======================
    let mut a = Point { x: 2, y: 4 };
    let b = &a.x;
    let c = &mut a.y;
    *c += 3;
    // a.x += 3; // this wont compile becaue b exists
    a.y += 3; // compiles cause c is not used after this point
    // println!("{:?} {:?} {:?}", a, b, c); // wont compile cause of c
    println!("{:?} {:?}", a, b);
    println!();

    // NOTE: we can consider 3 orders of partial borrowing fixes
    // 1. Change Order
    // 2. Change Area and fields of scope
    // 3. Change Data Layout

    let mut doc = DocumentOriginal { 
        text: String::from("checking something of things"),
        matches: Vec::new(),
        thumbnail: Thumbnail(0, 0)
    };

    doc.search_orig("thing");
    doc.search_orig_order("thing");
    doc.search_orig_area("thing");

    let mut doc_view = DocumentView {
        text: String::from("checking something for things in things"),
        view: View {
            matches: Vec::new(),
            thumbnail: Thumbnail(0, 0),
        }
    };

    doc_view.search_view("thing");


}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
#[allow(unused)]
struct Thumbnail(usize, usize);

#[derive(Debug)]
struct DocumentOriginal {
    text: String,
    matches: Vec<usize>,
    thumbnail: Thumbnail,
}

#[allow(unused)]
impl DocumentOriginal {
    // compiles
    fn search_orig(&mut self, txt: &str) {
        self.matches.clear();
        for (i, _) in self.text.match_indices(txt) {
            self.matches.push(i);
        }
    }

    // as of 1.99. the forloop will not compile
    // fn search(&mut self, txt: &str) {
    //     self.matches.clear();
    //     for (i, _) in self.text.match_indices(txt) {
    //         self.get_matched_highlights(i);
    //     }
    // }
    //
    // fn get_matched_highlights(&mut self, val: usize) {
    //     self.matches.push(val)
    //     // .....
    // }

    // compiles. We used the "changed order" for the following to fix borrowing from for loop
    fn search_orig_order(&mut self, txt: &str) {
        self.matches.clear();
        for (i, _) in self.text.match_indices(txt) {
            self.matches.push(i);
        }
        self.get_matched_highlights_order() // order change. after for loop
    }

    fn get_matched_highlights_order(&mut self) {
        // .....
    }

    // compiles. We used the "changed area" for the following to fix borrowing from for loop
    fn search_orig_area(&mut self, txt: &str) {
        self.matches.clear();
        for (i, _) in self.text.match_indices(txt) {
            Self::get_matched_highlights_area(
                &mut self.matches,
                i,
                &self.text,
                &self.thumbnail
            ) // send only fields
        }
    }

    fn get_matched_highlights_area(
        matches: &mut Vec<usize>,
        val: usize,
        str_lit: &str,
        thumb: &Thumbnail,
    ) {
        matches.push(val)
        // some stuff for thumbnails too
    }
}

#[allow(unused)]
#[derive(Debug)]
struct View {
    matches: Vec<usize>,
    thumbnail: Thumbnail,
}

#[derive(Debug)]
struct DocumentView {
    text: String,
    view: View,
}

#[allow(unused)]
impl DocumentView {
    // compiles. We used the "changed Layout" for the following to fix borrowing from for loop
    fn search_view(&mut self, txt: &str) {
        self.view.matches.clear();
        for (i, _) in self.text.match_indices(txt) {
            self.view.get_matched_highlight_view(i) // set separate types for view and text in loop
        }
    }

}

#[allow(unused)]
impl View {
    fn get_matched_highlight_view(&mut self, idx:usize) {
        self.matches.push(idx)
        // ..... thumbnails
    }
}
