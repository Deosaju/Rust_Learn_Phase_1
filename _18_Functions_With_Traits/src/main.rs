// functions with Traits

struct Data {
    some_vec: Vec<i32>
}
trait Basic_stats {
    fn mean(&self) -> f64;
    fn median(&self) -> f64;
    fn mode(&self) -> i32;
}

impl Basic_stats for Data {
    fn mean(&self) -> f64 {
        let mut sum = 0;
        for i in &self.some_vec {
            sum += i;
        }
        sum as f64 / self.some_vec.len() as f64
    }
    fn median(&self) -> f64 {
        let mut sorted = self.some_vec.clone();
        sorted.sort();
        let mid = sorted.len() / 2;
        if sorted.len() % 2 == 0 {
            (sorted[mid] + sorted[mid - 1]) as f64 / 2.0
        } else {
            sorted[mid] as f64
        }
    }
    fn mode(&self) -> i32 {
        let mut counts = std::collections::HashMap::new();
        for i in &self.some_vec {
            let count = counts.entry(i).or_insert(0);
            *count += 1;
        }
        let mut max = 0;
        let mut mode = 0;
        for (key, value) in counts {
            if value > max {
                max = value;
                mode = *key;
            }
        }
        mode
    }
}

fn main() {
    let data = Data { some_vec: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10] };
    println!("mean: {}", data.mean());
    println!("median: {}", data.median());
    println!("mode: {}", data.mode());
}
