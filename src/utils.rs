use winterfell::{
    math::fields::f128::BaseElement, Trace, TraceTable
};

pub(crate) fn display_trace(trace: &TraceTable<BaseElement>) {
    let mut target = Vec::<Vec<BaseElement>>::new();
    for i in 0..trace.width() {
        target.push(trace.get_column(i).to_vec());
    }

    for i in 0..trace.length() {
        print!("#");
        for j in 0..trace.width() {
            print!("{} ", target[j][i]);
        }
        println!("#");
    }
}