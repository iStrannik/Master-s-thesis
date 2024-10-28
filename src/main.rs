use my_fibo::{prover::prove_work, verifier::verify_work};
use winterfell::math::fields::f128::BaseElement;
use std::collections::HashMap;
use plotters::prelude::*;


fn main() {
    let mut memory = HashMap::new();
    let N = 15;
    

    let (fb_0, fb_1, n) = (BaseElement::new(2), BaseElement::new(2), 8);
    let (fb_n, proof) = prove_work(fb_0, fb_1, n, true);
    memory.insert(8, proof.to_bytes().len() / 1024);
    println!("{}", fb_n);
    println!("{}", proof.to_bytes().len() / 1024);
    verify_work(fb_0, fb_1, fb_n, proof);
    let mut x = Vec::new();
    let mut y = Vec::new();

    let mut previous_n: usize = 8;

    for i in 4..N {
        previous_n <<= 1;
        x.push(previous_n);
        let (fb_0, fb_1, n) = (BaseElement::new(2), BaseElement::new(2), previous_n);
        let (fb_n, proof) = prove_work(fb_0, fb_1, n, false);
        memory.insert(previous_n, proof.to_bytes().len() / 1024);
        y.push(proof.to_bytes().len() / 1024);
        println!("{}", i);
        println!("{}", fb_n);
        println!("{}", proof.to_bytes().len() / 1024);
        verify_work(fb_0, fb_1, fb_n, proof);
    }

    println!("################");

    let mut previous_n: usize = 8;

    for _ in 3..N {
        println!("{} {}", previous_n, memory.get(&previous_n).unwrap());
        previous_n <<= 1
    }


    let root_area = BitMapBackend::new("memory.png", (800, 600)).into_drawing_area();
    root_area.fill(&WHITE);

    let mut chart = ChartBuilder::on(&root_area)
        .caption("y = memory in kbytes", ("sans-serif", 40).into_font()) // Chart title
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0f32..((N+1) as f32), 0f32..100f32).unwrap();  // Smaller X and Y range

    // Configure the mesh (grid) and axis labels
    chart.configure_mesh()
        .x_labels(10)
        .y_labels(10)
        .disable_mesh()
        .draw();

    // Plot the parabola y = x^2
    chart.draw_series(LineSeries::new(x.iter().zip(y.iter()).map(|x| (f32::log2(*x.0 as f32), *x.1 as f32 )), &BLUE));

    // Save the chart to the file
    root_area.present();

    println!("Plot has been saved to 'memory.png'");
}
