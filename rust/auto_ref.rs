// Uniform Function Call Syntax (UFCS):
//   any function to be called using the syntax for method calls (as in object-oriented
//   programming), by using the *receiver* as the first parameter, and the given arguments
//   as the remaining parameters.

// For each each "dereference step" U (that is, set U = T and then U = *T, ...)
// 1. if there's a method |bar| where the *receiver* type (the type of self in the method)
//    matches U exactly , use it (a "by value method")
// 2. otherwise, add one auto-ref (take & or &mut of the receiver), and, if some method's receiver
//    matches &U, use it (an "autorefd method")
//
// Notably, everything considers the "receiver type" of the method, not the Self type of the trait,
// i.e. impl ... for Foo { fn method(&self) {} } thinks about &Foo when matching the method, and
// fn method2(&mut self) would think about &mut Foo when matching.


struct X { val: i32 }
impl std::ops::Deref for X {
    type Target = i32;
    fn deref(&self) -> &i32 { &self.val }
}

trait            M                   { fn m(self); }
impl             M for i32           { fn m(self) { println!("i32::m()"); } }
impl             M for X             { fn m(self) { println!("X::m()"); } }
impl<'a>         M for &'a X         { fn m(self) { println!("&X::m()"); } }
impl<'a, 'b>     M for &'a &'b X     { fn m(self) { println!("&&X::m()"); } }
impl<'a, 'b, 'c> M for &'a &'b &'c X { fn m(self) { println!("&&&X::m()"); } }

trait            RefM                   { fn refm(&self); }
impl             RefM for i32           { fn refm(&self) { println!("i32::refm()"); } }
impl             RefM for X             { fn refm(&self) { println!("X::refm()"); } }
impl<'a>         RefM for &'a X         { fn refm(&self) { println!("&X::refm()"); } }
impl<'a, 'b>     RefM for &'a &'b X     { fn refm(&self) { println!("&&X::refm()"); } }
impl<'a, 'b, 'c> RefM for &'a &'b &'c X { fn refm(&self) { println!("&&&X::refm()"); } }

struct Y { val: i32 }
impl std::ops::Deref for Y {
    type Target = i32;
    fn deref(&self) -> &i32 { &self.val }
}

struct Z { val: Y }
impl std::ops::Deref for Z {
    type Target = Y;
    fn deref(&self) -> &Y { &self.val }
}

#[derive(Copy, Clone)]
struct A;
impl             M for             A { fn m(self) { println!("A::m()"); } }
impl<'a, 'b, 'c> M for &'a &'b &'c A { fn m(self) { println!("&&&A::m()"); } }
impl             RefM for             A { fn refm(&self) { println!("A::refm()"); } }
impl<'a, 'b, 'c> RefM for &'a &'b &'c A { fn refm(&self) { println!("&&&A::refm()"); } }

fn main() {
    // I'll use @ to denote left side of the dot operator
    (*X{val:42}).m();        // i32::m()    , self == @
    X{val:42}.m();           // X::m()      , self == @
    (&X{val:42}).m();        // &X::m()     , self == @
    (&&X{val:42}).m();       // &&X::m()    , self == @
    (&&&X{val:42}).m();      // &&&X:m()    , self == @
    (&&&&X{val:42}).m();     // &&&X::m()   , self == *@
    (&&&&&X{val:42}).m();    // &&&X::m()   , self == **@

    println!("--------");

    (*X{val:42}).refm();     // i32::refm() , self == @
    X{val:42}.refm();        // X::refm()   , self == @
    (&X{val:42}).refm();     // X::refm()   , self == *@
    (&&X{val:42}).refm();    // &X::refm()  , self == *@
    (&&&X{val:42}).refm();   // &&X::refm() , self == *@
    (&&&&X{val:42}).refm();  // &&&X::refm(), self == *@
    (&&&&&X{val:42}).refm(); // &&&X::refm(), self == **@

    println!("--------");

    Y{val:42}.refm();        // i32::refm() , self == *@
    Z{val:Y{val:42}}.refm(); // i32::refm() , self == **@

    println!("--------");

    A.m();                   // A::m()      , self == @
    // without the Copy trait, (&A).m() would be a compilation error:
    // cannot move out of borrowed content
    (&A).m();                // A::m()      , self == *@
    (&&A).m();               // &&&A::m()   , self == &@
    (&&&A).m();              // &&&A::m()   , self == @
    A.refm();                // A::refm()   , self == @
    (&A).refm();             // A::refm()   , self == *@
    (&&A).refm();            // A::refm()   , self == **@
    (&&&A).refm();           // &&&A::refm(), self == @
}
