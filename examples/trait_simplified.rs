#[derive(Debug)]
struct One;
trait Item {
    fn label(&self) -> String;
}

impl Item for One {
    fn label(&self) -> String {
        format!("val is: {:?}", self )
    }
}

impl std::fmt::Display for One {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.label())
    }
}

fn describe<T>(item: &T)
where 
    T: std::fmt::Display + Item,
{
    println!("describe val {}", item.label());
}


#[derive(Debug)]
struct Two;

trait Item2: std::fmt::Display {
    const ST: &'static str;
    fn describe(&self);
}

impl std::fmt::Display for Two {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Item2 for Two {
    const ST:&'static str = "two";
    fn describe(&self){
        println!("{}: descrie 2 is {}", Self::ST, self);
    }
}

fn main() {
    Two.describe();

    let v = vec![1,2,3];
    // we can have many iters case of generic iter case
    #[allow(clippy::useless_conversion)]
    for x in v.into_iter().into_iter().into_iter() {
        println!("v item is {x}");
    }
}
