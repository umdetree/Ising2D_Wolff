use libm::exp;
use rand::Rng;

#[derive(Debug)]
pub struct Ising2D {
    _size: usize,
    _energy: f64,
    _sites: Vec<Vec<i8>>,
    _j: f64,
    _beta: f64,
    _mag_m: f64,
}

impl Ising2D {
    pub fn new(size: usize, j: f64, beta: f64) -> Ising2D {
        let mut ising_2d: Ising2D = Ising2D {
            _size: size,
            _energy: 0.0,
            _sites: vec![vec![0; size]; size],
            _j: j,
            _beta: beta,
            _mag_m: 0.0,
        };

        for m in 0..size {
            for n in 0..size {
                ising_2d._sites[m][n] = rand::thread_rng().gen_range(0..=1) * 2 - 1;
                ising_2d._mag_m += ising_2d._sites[m][n] as f64;
            }
        }

        for m in 0..size {
            for n in 0..size {
                ising_2d._energy += (ising_2d._sites[m][n] * ising_2d._sites[m][(n + 1) % size]
                    + ising_2d._sites[m][n] * ising_2d._sites[(m + 1) % size][n])
                    as f64;
            }
        }
        ising_2d._energy *= -j;

        return ising_2d;
    }

    pub fn get_energy(&self) -> f64 {
        return self._energy;
    }

    pub fn get_magnetic_momentum(&self) -> f64 {
        return self._mag_m / (self._size * self._size) as f64;
    }

    fn flip(&mut self, m: usize, n: usize) {
        let m = m % self._size;
        let n = n % self._size;

        self._sites[m][n] *= -1;
        self._mag_m += 2.0 * self._sites[m][n] as f64;
        self._energy += -2.0
            * self._j
            * (self._sites[m][n]
                * (self._sites[m][(n + 1) % self._size]
                    + self._sites[m][(n + self._size - 1) % self._size]
                    + self._sites[(m + 1) % self._size][n]
                    + self._sites[(m + self._size - 1) % self._size][n])) as f64;
    }

    pub fn flip_wolff(&mut self, m: usize, n: usize) {
        let m = m % self._size;
        let n = n % self._size;
        let p = 1.0 - exp(-2.0 * self._beta * self._j);

        let spin_0 = self._sites[m][n];
        self.flip(m, n);

        let mut dfs: Vec<(usize, usize)> = Vec::new();
        dfs.push((m, (n + 1) % self._size));
        dfs.push((m, (n + self._size - 1) % self._size));
        dfs.push(((m + 1) % self._size, n));
        dfs.push(((m + self._size - 1) % self._size, n));

        while !dfs.is_empty() {
            let (i, j) = dfs.pop().unwrap();

            if self._sites[i][j] * spin_0 == 1 {
                let rd = rand::thread_rng().gen_range(0.0..1.0);
                if rd < p {
                    self.flip(i, j);

                    dfs.push((i, (j + 1) % self._size));
                    dfs.push((i, (j + self._size - 1) % self._size));
                    dfs.push(((i + 1) % self._size, j));
                    dfs.push(((i + self._size - 1) % self._size, j));
                }
            }
        }
    }

    pub fn simulate_wolff(&mut self, steps: usize) {
        for _ in 0..steps {
            let pos_m = rand::thread_rng().gen_range(0..self._size);
            let pos_n = rand::thread_rng().gen_range(0..self._size);
            self.flip_wolff(pos_m, pos_n);
        }
    }
}
