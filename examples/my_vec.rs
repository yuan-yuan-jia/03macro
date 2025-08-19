use macros::my_vec;

fn main() -> anyhow::Result<()> {
    let v: Vec<i32> = my_vec![
        "1".parse()?,
        "2".parse()?,
        "3".parse()?,
        "4".parse()?,
        "5".parse()?,
    ];

    println!("{v:?}");

    Ok(())
}
