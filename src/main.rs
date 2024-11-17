fn main() {
    let country = String::from("Zimbabwe");
    let ref_one = &country;
    let _ref_two = &country;
    println!("{}", ref_one);
}
