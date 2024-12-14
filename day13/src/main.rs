use nalgebra::*;
use std::env;
use std::fs;
use regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let content = fs::read_to_string(args[1].clone()).expect("NO FILE");

    let mut sum: u128 = 0;
    let mut groups = content.split("\n\n");
    loop {
        let mut values: Vec<f64> = Vec::new();
        match groups.next() {
            Some(group) => {
                let mut builder: String = "".to_string();
                let mut chars = group.chars();
                loop {
                    match chars.next() {
                        Some(character) => {
                            if character.is_numeric(){
                                builder.push(character);
                            } else if character == ',' || character == '\n'{
                                values.push(builder.parse::<f64>().expect("did not parse to i32"));
                                builder = "".to_string();
                            }
                        }
                        None => {
                            if !builder.is_empty(){ 
                                values.push(builder.parse::<f64>().expect("did not parse to i32"));
                            }
                            break;
                        }
                    }
                }
            }
            None => {
                break;
            }
        }

        if !values.is_empty(){
            // Define matrix A (coefficients of the system)
            let a = DMatrix::from_row_slice(2, 2, &[values[0], values[2], values[1], values[3]]);
            // Define vector B (right-hand side of the system)
            let b = DVector::from_row_slice(&[values[4]+10000000000000.0, values[5]+10000000000000.0]);
            // Check if matrix A is invertible
            match a.try_inverse() {
                Some(a_inv) => {
                    let x = a_inv * b;
                    if (x[0] - x[0].round()).abs() < 0.0001 && (x[1] - x[1].round()).abs() < 0.0001{
                        sum += 3*x[0].round() as u128;
                        sum += x[1].round() as u128;
                    }
                }
                None => {}
            }
        }
    }
    println!("{}", sum);
}
