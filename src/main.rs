use plotters::prelude::*;

mod lattice;
use crate::lattice::Ising2D;

fn draw_vec(path: &str, data: &Vec<f64>, x_desc: &str, y_desc: &str, caption: &str) {
    let drawing_area = BitMapBackend::new(&path, (600, 400)).into_drawing_area();

    drawing_area.fill(&WHITE).unwrap();

    let mut max = f64::MIN;
    let mut min = f64::MAX;

    for num in data {
        if *num > max {
            max = *num;
        }
        if *num < min {
            min = *num;
        }
    }

    min *= 1.2;
    max *= 1.2;

    let mut chart = ChartBuilder::on(&drawing_area)
        .caption(caption, ("Arial", 30))
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .set_label_area_size(LabelAreaPosition::Left, 80)
        .build_cartesian_2d(0..data.len(), min..max)
        .unwrap();

    chart
        .configure_mesh()
        .x_desc(x_desc)
        .y_desc(y_desc)
        .draw()
        .unwrap();

    chart
        .draw_series(LineSeries::new((0..data.len()).map(|x| (x, data[x])), &RED))
        .unwrap();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 5 {
        eprintln!("Usage: ./ising2d <size> <J> <beta> <steps>");
        std::process::exit(1);
    }

    let size: usize = args[1].parse().expect("not a suitable <size>");
    let j: f64 = args[2].parse().expect("not a suitable <J>");
    let beta: f64 = args[3].parse().expect("not a suitable <beta>");
    let steps: usize = args[4].parse().expect("not a suitable <steps>");

    let mut lattice = Ising2D::new(size, j, beta);

    let mut e_res: Vec<f64> = vec![0.0; steps];
    let mut m_res: Vec<f64> = vec![0.0; steps];

    for i in 0..steps {
        e_res[i] = lattice.get_energy();
        m_res[i] = lattice.get_magnetic_momentum();
        lattice.simulate_wolff(1);
    }

    draw_vec(
        "./images/energy.png",
        &e_res,
        "steps",
        "Energy",
        "E in Wolff",
    );
    draw_vec(
        "./images/mag_m.png",
        &m_res,
        "steps",
        "magnetic momentum",
        "m in Wolff",
    );
}
