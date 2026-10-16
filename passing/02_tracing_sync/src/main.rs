#[tracing::instrument]
fn classify(value: u32) -> u32 {
    if value == 0 { 10 } else { 20 }
}

fn main() {
    assert_eq!(classify(0), 10);
}
