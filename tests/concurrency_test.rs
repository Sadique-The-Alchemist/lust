#[test]
fn check_websites_test() {
    let google = String::from("http://google.com");
    let gypsy = String::from("http://blog.gypsydave5.com");
    let smali = String::from("what://mmm.smali.ged");
    let websites = vec![google, gypsy, smali];

    let want = HashMap::from([(google, true), (gypsy, true), (smali, false)]);

    let got = check_websites(mock_website_checker, websites);
    assert_eq!(got, want, "expected{:?} but got {:?}", got, want)
}

fn mock_website_checker(url: String) -> bool {
    return url != "what://mmm.smali.ged";
}

// fn slow_stub_website_checker(url: &str) -> bool {
//     let stop_duration = time::Duration::from_millis(300);
//     thread::sleep(stop_duration);
//     return true;
// }
