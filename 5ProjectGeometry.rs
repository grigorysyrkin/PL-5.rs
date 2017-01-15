// wrap it into a loop -> pick the minimum

use std::io;
use std::f32;
use std::num;
// use std::from_str::from_str;


fn main() {
  let mut p1 = [0.0, 0.0];
  let mut p2 = [1.0, 1.0];
  
  let (a, b, c) = linePlot(&mut p1, &mut p2);
  println!("Line 1 plot coordinates: [{}, {}, {}]", a, b, c);
  
  let mut count = 0u32;
  let mut distance = 0f32;
  let mut shortestDistance = 1000f32;
  loop {
    count += 1;
// MARK:1
    let mut input_number = String::new();
    io::stdin()
      .read_line(&mut input_number)
      .expect("failed to read from stdin");
    
    let trimmed = input_number.trim();
    match trimmed.parse::<u32>() {//or u32
      Ok(i) => println!(""),
      Err(..) => println!("this was not an integer: {}", trimmed)
      };
    let result1: f32 = trimmed.parse().unwrap();
    
// MARK:2
    let mut input_number = String::new();
    io::stdin()
      .read_line(&mut input_number)
      .expect("failed to read from stdin");
    let trimmed = input_number.trim();
    match trimmed.parse::<u32>() {//or u32
      Ok(i) => println!(""),
      Err(..) => println!("this was not an integer: {}", trimmed)
      };
    let result2: f32 = trimmed.parse().unwrap();
  
    let mut p3: [f32; 2] = [result1, result2];
    // println!("{:?}", tup);

// MARK:3
    let mut input_number = String::new();
    io::stdin()
      .read_line(&mut input_number)
      .expect("failed to read from stdin");
    
    let trimmed = input_number.trim();
    match trimmed.parse::<u32>() {//or u32
      Ok(i) => println!(""),
      Err(..) => println!("this was not an integer: {}", trimmed)
      };
    let result1: f32 = trimmed.parse().unwrap();
    
// MARK:4
    let mut input_number = String::new();
    io::stdin()
      .read_line(&mut input_number)
      .expect("failed to read from stdin");
    let trimmed = input_number.trim();
    match trimmed.parse::<u32>() {//or u32
      Ok(i) => println!(""),
      Err(..) => println!("this was not an integer: {}", trimmed)
      };
    let result2: f32 = trimmed.parse().unwrap();
  
    let mut p4: [f32; 2] = [result1, result2];
    
    let (x, y, z) = linePlot(&mut p3, &mut p4);
    // println!("Line 2 plot coordinates: [{}, {}, {}]", x, y, z);

    let (res1, res2) = intersection((a, b, c), (x, y, z));
    let doubledDistance = ((p1[0] - res1).abs() * (p1[0] - res1).abs() + (p1[1] - res2).abs() * (p1[1] - res2).abs());
    let distance = doubledDistance.sqrt();
    if distance < shortestDistance {
      shortestDistance = distance;
    }
    // println!("{}", distance);
    if count == 2 {
      println!("shortestDistance: {}", shortestDistance);
      // println!("OK");
      break;
    }
  }
  
  
  // println!("{:?}", p1);
  let mut p3 = [0.0, 0.0];
  let mut p4 = [0.0, 1.0];

}

fn linePlot(p1: &mut [f32], p2: &mut [f32]) -> (f32, f32, f32) {
  let A = p1[1] - p2[1];
  let B = p2[0] - p1[0];
  let C = p1[0]*p2[1] - p2[0]*p1[1];
  return (A, B, -C);
}

fn intersection((a, b, c):(f32, f32, f32), (x, y, z):(f32, f32, f32)) -> (f32, f32) {
  let D = a * y - b * x;
  let Dx = c * y - b * z;
  let Dy = a * z - c * x;
  if D != 0.0 {
    let resultX = Dx / D;
    let resultY = Dy / D;
    return (resultX, resultY);
  }
  else {
    return (999.9, 999.9);
  }
}