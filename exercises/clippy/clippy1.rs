// clippy1.rs
// The Clippy tool is a collection of lints to analyze your code
// so you can catch common mistakes and improve your Rust code.
//
// For these exercises the code will fail to compile when there are clippy warnings
// check clippy's suggestions from the output to solve the exercise.
// Execute `rustlings hint clippy1` for hints :)

use std::f32;

fn main() {
    let radius = 5.00f32;

    let area = f32::consts::PI * f32::powi(radius, 2);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    )
}

// Solution for 4.6.0 release
// fn main() {
//     let x = 1.2331f64;
//     let y = 1.2332f64;
//     if (y - x).abs() != 0.0 {
//         println!("Success!");
//     }
// }
