fn main() {
    let config_max = Some(3_u8);
    // match config_max {
    //     Some(max) => println!("The max configs are {}", max),
    //     None => ()
    // }

    if let Some(max) = config_max {
        println!("The max configs are {}", max)
    } // We an give else and all here as well
      // Non exhaustive in nature

}
