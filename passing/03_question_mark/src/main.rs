fn convert(input: Result<u32, &'static str>) -> Result<u32, &'static str> {
    let value = input?;
    Ok(value + 1)
}

fn main() {
    assert_eq!(convert(Err("missing")), Err("missing"));
    assert_eq!(convert(Ok(4)), Ok(5));
}
