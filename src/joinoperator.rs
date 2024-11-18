extern crate test;

use test::Bencher;

fn strings_vec() -> Vec<String> {
    vec![String::from("Rustling"); 512]
}

#[bench]
fn bench_join(b: &mut Bencher) {
    let words = strings_vec();
    b.iter(|| {
        let _ = words.join("-");
    });
}

#[bench]
fn bench_connect(b: &mut Bencher) {
    let words: Vec<String> = strings_vec();
    b.iter(|| {
        let _ = words.connect("-");
    });
}
