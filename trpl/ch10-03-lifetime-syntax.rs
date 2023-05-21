fn main() {
    tes1();
    tes2();
}

fn tes1() {
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let result = longest1(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);
}

fn tes2() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        // result = longest1(string1.as_str(), string2.as_str()); faild
        result = longest2(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}

fn longest1<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
    println!("y {}", y);
    x
}
