
#[derive(Debug,Copy, Clone)]
struct Set {
    domain: i32,
    range: i32,
}
#[derive(Debug)]
struct Fnsets {
    isfn: bool,
    sets: Vec<Set>,
}

impl Fnsets {
    fn new(domain: i32, range: i32) -> Self {
        Self {isfn: true,
            sets: vec![Set{
                domain,
                range,
                }]}
    }
    fn add(&mut self, domain: i32, range: i32){ 
        let cset: Set = Set{domain, range};
        if self.isfn == false { 
            self.sets.push(cset);
            return };
        for set in &self.sets {
            if set.domain == cset.domain {
                if set.range != cset.range {
                self.isfn = false;
                }
            }
        }
        self.sets.push(cset);
        // self.sets.push(Set{ 
        //     domain: domain,
        //     range: range,
        // });
    

    }
    // fn isfun(self) {
    //     for s in fnsets {
    //         for r in s {
    //             //see if 2 sets are equal
    //         }
    //     }
    // }  
}
fn main() {
    let mut fnel: Fnsets = Fnsets::new(1,2);
    fnel.add(6,4);
    fnel.add(1,4);
    fnel.add(1,5);

    println!("fnel: {:?}", fnel);
    println!("isfn: {:?}", fnel.isfn);
}
