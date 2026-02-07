let hello = "Hello World";
let num: i32 = 200;

let signed_int8: i8 = -128;
let signed_int16: i16 = -32768;
let signed_int32: i32 = -2147483648;
let signed_int64: i64 = -9223372036854775808;
let signed_int128: i128 = -170141183460469231731687303715884105728;
let signed_isize: isize = -1; 

let unsigned_int8: u8 = 255;
let unsigned_int16: u16 = 65535;
let unsigned_int32: u32 = 4294967295;
let unsigned_int64: u64 = 18446744073709551615;
let unsigned_int128: u128 = 340282366920938463463374607431768211455;
let unsigned_usize: usize = 1;

let float32: f32 = 3.14159;
let float64: f64 = 3.141592653589793;

let boolean_true: bool = true;
let boolean_false: bool = false;

let character: char = 'A';

let string_slice: &str = "This is a string slice";
let owned_string: String = String::from("This is an owned string");

let array: [i32; 5] = [1, 2, 3, 4, 5]; 
let tuple: (i32, f64, char) = (42, 3.14, 'Z'); 

println!("Hello: {}", hello);
println!("Num: {}", num);
println!("Signed i8: {}", signed_int8);
println!("Float64: {}", float64);
println!("Boolean: {}", boolean_true);
println!("Char: {}", character);
println!("String: {}", owned_string);
println!("Array: {:?}", array);
println!("Tuple: {:?}", tuple);
