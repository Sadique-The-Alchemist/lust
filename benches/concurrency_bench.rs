#![feature(test)]

extern crate test;

use test::Bencher;

// Import the function to be benchmarked
use lust::concurrency::concurrency::check_websites; // Replace with your crate name

#[bench]
fn check_websites_bench(b: &mut Bencher) {
    let google = String::from("http://google.com");
    let gypsy = String::from("http://blog.gypsydave5.com");
    let smali = String::from("what://mmm.smali.ged");
    let websites = vec![google, gypsy, smali];

    b.iter(|| check_websites(mock_website_checker, websites.clone()));
}

fn mock_website_checker(url: String) -> bool {
    return url != "what://mmm.smali.ged";
}
