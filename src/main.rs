use plotters::prelude::*;
use std::io::Write;

mod lattice;
use crate::lattice::Ising2D;

fn draw_data(path: &str, x_data: &Vec<f64>, y_data: &Vec<f64>, x_desc: &str, y_desc: &str, caption: &str) {
    let sz = usize::min(x_data.len(), y_data.len());
    
    let drawing_area = BitMapBackend::new(&path, (600, 400)).into_drawing_area();

    drawing_area.fill(&WHITE).unwrap();

    let mut x_max = f64::MIN;
    let mut x_min = f64::MAX;
    let mut y_max = f64::MIN;
    let mut y_min = f64::MAX;

    for i in 0..sz {
        x_max = f64::max(x_max, x_data[i]);
        x_min = f64::min(x_min, x_data[i]);
        y_max = f64::max(y_max, y_data[i]);
        y_min = f64::min(y_min, y_data[i]);
    }

    x_min *= 1.2;
    x_max *= 1.2;
    y_min *= 1.2;
    y_max *= 1.2;

    let mut chart = ChartBuilder::on(&drawing_area)
        .caption(caption, ("Arial", 30))
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .set_label_area_size(LabelAreaPosition::Left, 80)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)
        .unwrap();

    chart
        .configure_mesh()
        .x_desc(x_desc)
        .y_desc(y_desc)
        .draw()
        .unwrap();

    chart
        .draw_series((0..sz).map(|i| Circle::new((x_data[i], y_data[i]), 2, RED.filled())))
        .unwrap();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 6 {
        eprintln!("Usage: ./ising2d <size> <J> <beta_start> <beta_end> <mc_times>");
        eprintln!("<size>: The base lattice length used in Binder cumulant simulation");
        eprintln!("<J>: Need not to explain. Set to 1.0 in M-L simulaiton");
        eprintln!("<beta_start/end>: Temperature range. (Note that Tc=2.269 when J=1,KB=1)");
        eprintln!("<mc_times>: Monte Carlo simulation times in Binder cumulant and M-L");

        std::process::exit(1);
    }

    let size: usize = args[1].parse().expect("not a suitable <size>");
    let j: f64 = args[2].parse().expect("not a suitable <J>");
    let beta_start: f64 = args[3].parse().expect("not a suitable <beta_start>");
    let beta_end: f64 = args[4].parse().expect("not a suitable <beta_end>");
    let mc_times: usize = args[5].parse().expect("not a suitable <mc_times>");

    println!("----------Simulation parameters---------");
    println!("In Binder cumulant simulation, length: {0}, {1}, {2}, J: {3}", size*2, size, size/2, j);
    println!("In M-L simulation, J=1.0, KB=1.0, Tc=2.269185, lattice length: [10,20,30,40,50]");
    println!("beta range: [{0}, {1}]", beta_start, beta_end);
    println!("Monte Carlo simulation times: {0}", mc_times);
    println!("-----------Start simulation-------------");
    println!("-------Binder cumulant simulation-------");

    for _ in 0..100 {
        print!("+");
    }
    println!("");

    let mut betas: Vec<f64> = vec![0.0; 100];
    let mut m_res: Vec<Vec<f64>> = vec![vec![0.0; 100]; 3];
    let mut u_res: Vec<Vec<f64>> = vec![vec![0.0; 100]; 3];
    let sizes: Vec<usize> = vec![size/2, size, size*2];

    let beta_step = (beta_end - beta_start) / 100.0;
    for s in 0..3 {
        for i in 0..100 {
            if (s * 100 + i) % 3 == 0 {
                print!("*");
                std::io::stdout().flush().unwrap();
            }

            betas[i] = beta_start + beta_step * i as f64;

            let mut m_tot = 0.0;
            let mut m2_tot = 0.0;
            let mut m4_tot = 0.0;
        
            for _ in 0..mc_times {
                let mut lattice = Ising2D::new(sizes[s], j, betas[i]);
                lattice.simulate_wolff(sizes[s] * sizes[s]);
                let m = lattice.get_magnetic_momentum().abs();
                m_tot += m;
                m2_tot += m*m;
                m4_tot += m*m*m*m;
            }

            m_res[s][i] = m_tot / mc_times as f64;
            let m2 = m2_tot / mc_times as f64;
            let m4 = m4_tot / mc_times as f64;
            u_res[s][i] = 1.5 * (1.0 - m4 / (3.0 * m2* m2));
        }
    }

    // Draw data
    let ts :Vec<f64> = betas.iter().map(|b| 1.0/b).collect();

    let drawing_area = BitMapBackend::new("./images/u4.bmp", (600, 400)).into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();

    let mut x_max = f64::MIN;
    let mut x_min = f64::MAX;
    let mut y_max = f64::MIN;
    let mut y_min = f64::MAX;

    for i in 0..100 {
        x_max = f64::max(x_max, ts[i]);
        x_min = f64::min(x_min, ts[i]);
        y_max = f64::max(y_max, u_res[0][i]);
        y_min = f64::min(y_min, u_res[0][i]);
    }
    x_min *= 1.2;
    x_max *= 1.2;
    y_min *= 1.2;
    y_max *= 1.2;

    let mut chart = ChartBuilder::on(&drawing_area)
        .caption("u4", ("Arial", 30))
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .set_label_area_size(LabelAreaPosition::Left, 80)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)
        .unwrap();
    
    chart
        .configure_mesh()
        .x_desc("t")
        .y_desc("u")
        .draw()
        .unwrap();

    for i in 0..3 {
        chart
            .draw_series(LineSeries::new((0..100).map(|k| (ts[k], u_res[i][k])), HSLColor(0.2 * i as f64, 1.0, 0.5).filled()))
            .unwrap();
        drawing_area.present().expect("failed to draw");
    }
    println!("\n--Binder cumulant simulation complete---");


    println!("-----------m-L simulation---------------");
    const T_C: f64 = 2.269185;

    let mut ls: Vec<usize> = vec![0; 5];
    for i in 0..5 {
        ls[i] = i * 10 + 10;
    }

    let mut x_res: Vec<f64> = vec![0.0; 500];
    let mut y_res: Vec<f64> = vec![0.0; 500];

    let mut betas: Vec<f64> = vec![0.0; 100];
    for (i, b) in betas.iter_mut().enumerate() {
        *b = beta_start + (beta_end - beta_start) / 100.0 * i as f64;
    }

    for _ in 0..50 {
        print!("+");
    }
    println!("");

    for i in 0..5 {
        for k in 0..100 {
            if k % 10 == 0 {
                print!("*");
                std::io::stdout().flush().unwrap();
            }
            let mut m_tot = 0.0;
            for _ in 0..mc_times {
                let mut lattice = Ising2D::new(ls[i], j, betas[k]);
                lattice.simulate_wolff(ls[i] * ls[i]);
                m_tot += lattice.get_magnetic_momentum().abs();
            }
            x_res[i * 100 + k] = ls[i] as f64 * (1.0 / betas[k] - T_C) / T_C;
            y_res[i * 100 + k] = m_tot / mc_times as f64 * (ls[i] as f64).powf(0.125);
        }
    }
    draw_data(
        "./images/scaling.png",
        &x_res,
        &y_res,
        "x",
        "y",
        "scaling",
    );
    println!("\n--------------Complete------------------");

}
