use my_fibo::{prover::prove_work, verifier::verify_work, air::COUNT};
use winterfell::math::fields::f128::BaseElement;
use std::{env, fs::OpenOptions};
use plotters::prelude::*;
use std::io::prelude::*;



fn main() {
    let args: Vec<_> = env::args().collect();

    let fb_start = [BaseElement::new(1); COUNT];

    if args.iter().any(|i| i == "debug") {
        let n = 8;
        let (fb_n, proof) = prove_work(fb_start, n, true);
        println!("n-th fibo is {}", fb_n);
        println!("proof size is {}", proof.to_bytes().len() / 1024);
        verify_work(fb_start, fb_n, proof);
    }
    
    let n = 18;
    let mut x = Vec::new();
    let mut y = Vec::new();
    let mut previous_n: usize = 8;

    for _ in 4..n {
        previous_n <<= 1;
        x.push(previous_n);
        let (fb_n, proof) = prove_work(fb_start, previous_n, false);
        y.push(proof.to_bytes().len() / 1024);
        println!("current n is {} fibo_n is {} memory used is {}", previous_n, fb_n, proof.to_bytes().len() / 1024);
    }

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("columns_to_bytes.txt")
        .unwrap();

    if let Err(e) = writeln!(file, "{} {}", COUNT, y.last().unwrap()) {
        eprintln!("Couldn't write to file: {}", e);
    }

    println!("################");

    let filename = format!("memory_{}.png", COUNT);

    let binding = filename.clone();
    let root_area = BitMapBackend::new(&binding, (800, 600)).into_drawing_area();
    let _ = root_area.fill(&WHITE);

    let mut chart = ChartBuilder::on(&root_area)
        .caption("y = memory in kbytes", ("sans-serif", 40).into_font()) // Chart title
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0f32..((n+1) as f32), 0f32..(*y.last().unwrap() + 10)as f32).unwrap();  // Smaller X and Y range

    let _ = chart.configure_mesh()
        .x_labels(10)
        .y_labels(10)
        .disable_mesh()
        .draw();

    let _ = chart.draw_series(LineSeries::new(x.iter().zip(y.iter()).map(|x| (f32::log2(*x.0 as f32), *x.1 as f32 )), &BLUE));

    // Save the chart to the file
    let _ = root_area.present();

    println!("Plot has been saved to '{}'", filename);
}
