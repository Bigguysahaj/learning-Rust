fn main() {
    println!("Hello, world!");

    let x: i32 = 5;
    // i8, i16, i32, i64, i128, isize are data types of integers
    // u8, u16, u32, u64, u128, usize are data types of unsigned integers
    

    // FLOATING POINTS
        let flt_point: f32 = 5.0;
        // f64 is default floating point type
        // f32, f64 are data types of floating points
        // f32 is 32 bits, f64 is 64 bits


    // BOOLEAN
        let true_bool: bool = true;

    // Character
        let char: char = 'a';
        // char is 4 bytes, and can store unicode characters
        // char is a single character, not a string

    //compound types
    // TUPLE
        let mut tup: (i32, bool, char) = (1,true,'s');
        let tup2: (i8, bool, char) = (1, true, 'a');
        tup = (10, false, 'a');
        // these 2 tuple are different because of the diff data types
        println!("tup is = {}", tup.0);

        // cant add elements to tuple, but can change the values

}
