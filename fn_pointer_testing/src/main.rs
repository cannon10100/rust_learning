type PotentialFunc = fn(&[String]) -> i32;

struct Holder<'a> {
    names: &'a [String],
    func: PotentialFunc
}

#[derive(Debug)]
struct Larger<'a> {
    holders: Vec<Holder<'a>>,
}

impl<'a> std::fmt::Debug for Holder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Holder {{ {:?}, <func: {}> }}", self.names, (self.func)(self.names))
    }
}

impl<'a> Larger<'a> {
    fn new() -> Larger<'a> {
        Larger { holders: vec!() }
    }

    fn add_holder(&mut self, holder: Holder<'a>) {
       self.holders.push(holder); 
    }
}

fn dummy_func(args: &[String]) -> i32 {
   args.len() as i32 
}


fn main() {
    let h = Holder { names: &vec!(String::from("Hello"), String::from("world!")), func: dummy_func };
    let mut larger = Larger::new();
    larger.add_holder(h);

    println!("{:?}", larger);
}
