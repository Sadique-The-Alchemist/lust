pub mod concurrency {
    use std::collections::HashMap;
    use std::sync::mpsc;
    use std::thread;

    type WebsiteChecker = fn(String) -> bool;

    pub fn check_websites(wc: WebsiteChecker, urls: Vec<String>) -> HashMap<String, bool> {
        let mut results: HashMap<String, bool> = HashMap::new();

        let (tx, rx) = mpsc::channel();

        for url in urls {
            let tx = tx.clone();
            thread::spawn(move || {
                let is_valid = wc(url.clone());
                tx.send((url, is_valid)).expect("Failed to send");
            });
        }
        drop(tx);

        for (url, result) in rx {
            results.insert(url, result);
        }

        return results;
    }

    // pub fn fibonacci(n: u32) -> u32 {
    //     let mut a = 0;
    //     let mut b = 1;
    //     for _ in 0..n {
    //         let tmp = a + b;
    //         a = b;
    //         b = tmp;
    //     }
    //     a
    // }
}
