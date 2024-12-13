pub mod sum {
    pub fn sum(numbers: [i32; 5]) -> i32 {
        let mut sum: i32 = 0;
        for n in numbers {
            sum += n;
        }
        return sum;
    }
    pub fn sum_vec(numbers: Vec<i32>) -> i32 {
        let sum = numbers.iter().sum();

        return sum;
    }
    pub fn sum_slice(numbers: &[i32]) -> i32 {
        return numbers.iter().sum();
    }

    pub fn sum_all(numbers: &[&[i32]]) -> Vec<i32> {
        let mut totals: Vec<i32> = Vec::new();
        for ns in numbers {
            let nsm = sum_slice(ns);
            totals.push(nsm);
        }
        return totals;
    }
}
