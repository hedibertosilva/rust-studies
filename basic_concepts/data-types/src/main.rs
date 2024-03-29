fn main() {

    let result:f64 = (0.1+0.7) * 10.0;

    println!("{}", result); // 7.99999999999
}

/*

    Scalar Types

    * Integers

        Signed = -2^(n-1) ~ +2^(n-1)-1
        Unsigned = 0 ~ 2^(n)-1

        Length  Signed  Unsigned
        8-bit   i8      u8
        16-bit  i16     u16
        32-bit  i32     u32
        64-bit  i64     u64
        128-bit i128    u128
        arch    isize   usize

        Number literals     Example
        Decimal             98_222
        Hex                 0xff
        Octal               0o77
        Binary              0b1111_0000
        Byte (u8 only)      b'A'
    
    * Floating-Point

        There are two primitive types for floating-point numbers.

        f32 = 32 bits -> single-precision
        f64 = 64 bits -> double-precision

        fn main() {
            let a:f32 = 2.0;
            let b:f64 = 2.2;
            let c = 2.4; // default f64
        }

    * Numeric Operations

        fn main() {
            // addition
            let sum = 5 + 10;

            // subtraction
            let difference = 95.5 - 4.3;

            // multiplication
            let product = 4 * 30;

            // division
            let quotient = 56.7 / 32.2;

            // remainder
            let remainder = 43 % 5;
        }

    * Booleans = 8 bits = 1 byte

        fn main() {
            let t = true;

            let f: bool = false; // with explicit type annotation
        }

    * Characters = 32 bits = 4 bytes

        char literals are specified with single quotes, as opposed to string literals, which use double quotes.

        fn main() {
            let c = 'z';
            let z = 'ℤ';
            let heart_eyed_cat = '😻';
        }


    Compound Types

    * Tuple
        It's fixed lenght. 

        fn main() {
            let tup: (i32, f64, u8) = (500, 6.4, 1);
        }

        It's possible to destructure a tup.

        fn main() {
            let example = (1, 2, 3);
            let (x, y, z) = example;

            println!("{}", x); // 1
            println!("{}", y); // 2
            println!("{}", z); // 3
        }

        It's possible to access the values directly.

        fn main() {
            let example = (1, 2, 3);
            
            println!("{}", example.0); // 1
            println!("{}", example.1); // 2
            println!("{}", example.2); // 3
        }

    * Array

        All values in array must have the same type. 
        Arrays in Rust are different  from arrays in some other languages because
        arrays in Rust have a fixed lenght, like tuples.

        fn main() {
            let a[u8; 5] = [1, 2, 3, 4, 5];
            let first = a[0];
            let second = a[1];
        }

        Obs! dont accept negative nubmers to access the array!

        ** Generating Arrays

        fn main() {
            let _a = [0; 5]; // [0, 0, 0, 0, 0]
        }

        fn main() {
            let full_name = ["hediberto", "cavalcante", "da", "silva"];

            println!("Please type a index:");

            let mut index = String::new();

            std::io::stdin()
                    .read_line(&mut index)
                    .expect("Please type a valid index!");

            let index:usize = index.trim().parse().expect("Type a valid index, please!");

            println!("The index {} is equal to value {}.", index, full_name[index]);

        }


*/