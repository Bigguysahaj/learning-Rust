fn main() {
    let mut x = 4;
    // let x = "hello"; dosen't work because x is already defined as int
    println!(" x is = {}", x);
    // {} is formating syntax

    // scope and name shadowing
    {
        // interior
        let x = x - 2;
        // x is only defined in this scope, and only equal 
        // to 2 in this scope
        println!(" y is = {}", x);
    }

    let x = x + 1;
    let x = "hello";
    println!(" x is = {}", x);

    // const defining, can't redefine again and again.
    const SECONDS_IN_MINUTE: u32 = 60;
    println!(" seconds in a minute = {}", SECONDS_IN_MINUTE);

}
