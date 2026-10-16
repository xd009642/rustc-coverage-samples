use thiserror::Error;

#[derive(Debug, Error)]
#[error("{}", if *.0 { "selected" } else { "untaken" })]
struct Failure(bool);

fn main() {
    assert_eq!(Failure(true).to_string(), "selected");
}
