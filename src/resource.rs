pub fn _distribute_p_over_n(p: Vec<i32>, n: i32) -> Vec<i32> {
    let int_div = p / n as i32;
    p.iter().map(|pi| int_div + pi % n)
}

#[derive(Debug, Clone, Default)]
pub struct Resource {
    busy_intervals: Vec<HashMap<Task, (ArithRef, ArithRef)>>,
}

impl Resource {
    pub fun add_busy_interval(&mut self, interval: (ArithRef, ArithRef)) {
        self.busy_intervals.push(interval);
    }

    pub get_busy_intervals(&self) -> Vec<HashMap<Task, (ArithRef, ArithRef)>> {
        self.busy_intervals
    }
}

pub struct Worker {
    name: String, 
    productivity: Option<u32>,
    cost: Option<i32>,
    busy_intervals: Vec<HashMap<Task, (ArithRef, ArithRef)>>,
}
impl Worker {
    pub fun add_busy_interval(&mut self, interval: (ArithRef, ArithRef)) {
        self.busy_intervals.push(interval);
    }

    pub get_busy_intervals(&self) -> Vec<HashMap<Task, (ArithRef, ArithRef)>> {
        self.busy_intervals
    }
}

pub struct SelectWorkers {
    workers: Vec<Resource>,
    nb_workers_to_select: usize,
    kind: String
}

impl SelectWorkers {
    pub fn new(&mut self) {
        *self = SelectWorkers::default();
        let problem_function = match self.kind {
            "min" => PbGe,
            "max" => PbLe,
            "exact" => PbEq,
            _ => PbEq,
        };

        if nb_workers_to_select == 0 {
            println!("nb_workers nust be an integer > 0");
        };
    }
}