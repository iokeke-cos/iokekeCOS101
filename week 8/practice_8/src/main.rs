fn main() {
    //initailize a mutable tuple
    let mut mountain_heights = ("Everest", 8848, "FIshtail", 6993);

    println!("Original tuple = {:?}",mountain_heights);

    //change 3rd and 4th element of mutable tuple
    mountain_heights.2 = "Lhotse";
    mountain_heights.3 = 8516;

    println!("Changed tuple = {:?}",mountain_heights);
}
