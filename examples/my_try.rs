use macros::my_try;

fn main() -> anyhow::Result<()> {
    let ret = my_try!(f3(my_try!(f2(my_try!(f1("hello"))))));
    println!("Final result: {ret}");
    Ok(())
}

fn f1(s: impl AsRef<str>) -> anyhow::Result<String> {
    Ok(format!("f1:{}", s.as_ref()))
}

fn f2(s: impl AsRef<str>) -> anyhow::Result<String> {
    Ok(format!("f2:{}", s.as_ref()))
}

fn f3(s: impl AsRef<str>) -> anyhow::Result<String> {
    Ok(format!("f3:{}", s.as_ref()))
}
