fn main() {
    let binary_number = "1111";
    let conversion = isize::from_str_radix(binary_number, 2).unwrap();
    println!("{}", conversion)  
}
    