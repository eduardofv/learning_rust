pub struct AveragedCollection {
    values: Vec<i32>,
    mean: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, val: i32) {
        self.values.push(val);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let res = self.values.pop(); 
        match res {
            Some(res) => {
                self.update_average();
                Some(res)
            },
            None => None,
        }
    }

    pub fn average(&mut self) -> f64 {
        return self.mean;
    }

    fn update_average(&mut self) {
        let sum: i32 = self.values.iter().sum();
        self.mean = sum as f64 / self.values.len() as f64;
    }
}


