// See TODO at the very bottom.
//
// This Slab type implements a collection based
// on a fixed-length array. It can hold up to N
// items, with each entry in the array either
// occupied or vacant.
// 
// Implement Index and IndexMut so a Slab can be
// indexed using square brackets with a usize index.
//
// Accessing a vacant index should panic, just like
// indexing beyond the end of an array panics.

use std::array; // for array::from_fn
use std::mem;   // for mem::replace

// NOTE: If we have a struct that is public
// but it wraps a type that is private, we have to 
// careful how we expose the private type.
// Meaning any function or impl signatures show that
// private type, the compiler could throw a warning/error.
// However, if the wrapper, being public, doesnt 
// expose the private type, even if the private type
// is encased in a public type which can be the output,
// is will be allowed to compile.
// For example, if I have this block below. I would 
// complile. However parts throw warning and if used
// in a pub traits such as Index, this wont compile
// because its a pub traits, not just a warning we 
// get for pub functions.
/*
pub struct Val<T, const N: usize> {
    myval: [Other<T>; N],
}

enum Other<T> {
    Take(T),
    None,
}

impl Val<u8, 5> {
    pub fn new() -> Self {
        Val { myval: [ Other::Take(1), Other::None, Other::None, Other::None, Other::None] }
    }

    // WARNING: This will compile as we Other is the return type
    // but it will throw a warning for using private times in
    // signature for pub method. This attribute is to allow the 
    // linter to not give us the warning messages
    // This is allow a questionable API implementation without
    // much purpose
    #[allow(private_interfaces)]
    pub fn get(idx: usize) -> Other<u8> {
        todo!()
    }

    // WARNING: This will compile as we Other is the return type
    // but it will throw a warning for using private times in
    // signature for pub method.
    // This is allow a questionable API implementation without
    // much purpose
    pub fn comp(val: Other<u8>) {
        todo!()
    }
}

// NOTE: this wont compile as Index bring a pub trait exposes Output.
// Meaning this exposes the relationship that this trait uses the
// Output type. But that being private will be denied by the 
// compiler case we cant something like: let x = Val....index()
// Also meaning, Index is different because the trait is essentially
// providing behavious for that type ( a type this is private ).

// impl<T, const N: usize> std::ops::Index<usize> for Val<T, N> {
//     type Output = Other<T>; // this wont compile
//
//     fn index(&self, index: usize) -> &Self::Output {
//         &self.myval[index]
//     }
// }
*/

enum Entry<T> {
    Occupied(T),
    Vacant(usize),
}

pub struct Slab<T, const N: usize> {
    buf: [Entry<T>; N],
    head: usize,
}

impl<T, const N: usize> Slab<T, N> {
    pub fn new() -> Self {
        Slab {
            buf: array::from_fn(|i| Entry::Vacant(i + 1)),
            head: 0,
        }
    }

    pub fn insert(&mut self, item: T) -> Result<usize, T> {
        let index = self.head;
        if index == N {
            return Err(item);
        }
        let Entry::Vacant(next) = self.buf[index] else {
            panic!("head must be vacant");
        };
        self.buf[index] = Entry::Occupied(item);
        self.head = next;
        Ok(index)
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        if let Entry::Vacant(_) = self.buf[index] {
            return None;
        }
        let Entry::Occupied(item) = mem::replace(
            &mut self.buf[index],
            Entry::Vacant(self.head),
        ) else {
            unreachable!();
        };
        self.head = index;
        Some(item)
    }

    /// Get a shared reference to the item at `index`.
    ///
    /// Returns `None` if the entry is vacant.
    pub fn get(&self, index: usize) -> Option<&T> {
        match &self.buf[index] {
            Entry::Occupied(item) => Some(item),
            Entry::Vacant(_) => None,
        }
    }

    /// Get a mutable reference to the item at `index`.
    ///
    /// Returns `None` if the entry is vacant.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        match &mut self.buf[index] {
            Entry::Occupied(item) => Some(item),
            Entry::Vacant(_) => None,
        }
    }
}

#[test]
#[should_panic]
fn test_index_panic() {
    #[derive(Debug)]
    struct Foo; // private type prevents cheating

    let mut slab: Slab<Foo, 5> = Slab::new();
    let i = slab.insert(Foo).unwrap();
    slab.remove(i).unwrap();
    let _ = slab[i]; // panic
}

#[test]
#[should_panic]
fn test_index_mut_panic() {
    let mut slab: Slab<char, 5> = Slab::new();
    slab[0] = 'x'; // panic
}

#[test]
fn test() {
    let mut slab: Slab<char, 3> = Slab::new();
    let i = slab.insert('a').unwrap();

    let a = slab[i]; // uses Index
    assert_eq!(a, 'a');
    slab[i] = 'b'; // uses IndexMut
    slab[i].make_ascii_uppercase(); // uses IndexMut
    let b = slab[i]; // uses Index
    assert_eq!(b, 'B');
}

// No need to change anything above this line.
// -------------------------------------------------------

// TODO: impl Index
impl<T, const N: usize> std::ops::Index<usize> for Slab<T, N> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        let Entry::Occupied(val) = &self.buf[index] else {
            panic!("value not Occupied for the index val");
        };
        val
    }
}

// TODO: impl IndexMut
impl<T, const N: usize> std::ops::IndexMut<usize> for Slab<T, N> {
    //NOTE: we can have trait inheritance, meaning we inherit
    //Ouput from Index cause IndexMut: Index
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let Entry::Occupied(val) = &mut self.buf[index] else {
            panic!("value not Occupied for the mut index val");
        };
        val
    }
}
