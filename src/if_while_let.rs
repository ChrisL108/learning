// This enum purposely neither implements nor derives PartialEq.
// That is why `if Foo::Bar == a` fails
enum Foo {
    Bar,
}
fn main() {
    // * IF LET
    let a = Foo::Bar;
    // Variable a matches Foo::Bar
    if let Foo::Bar = a {
        println!("a is foobar");
    }
    // * WHILE LET
    //? verbose "if let" examp. that can be
    //? more concise with "while let"
    let mut optional = Some(0);
    loop {
        match optional {
            Some(v) => {
                if v > 9 {
                    println!("Finished 'match'");
                    optional = None;
                } else {
                    println!("Counting.. {:?}", v);
                    optional = Some(v + 1);
                }
            }
            _ => {
                break;
            }
        }
    }
    let mut optional2 = Some(0);
    while let Some(v) = optional2 {
        if v > 9 {
            println!("Finished 'while let'");
            optional2 = None;
        } else {
            println!("Counting.. {:?}", v);
            optional2 = Some(v + 1);
        }
    }
}
