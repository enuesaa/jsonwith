use crate::json::parts::Parts;

#[derive(Debug)]
struct ScalarJudger {
    vals: Vec<char>,
}
impl ScalarJudger {
    pub fn new() -> Self {
        ScalarJudger { vals: Vec::new() }
    }

    fn add_val(&mut self, val: &char) {
        self.vals.push(*val);
    }

    fn judge(self) -> bool {
        let vals: String = self.vals.into_iter().collect();
        println!("{:?}", vals);
        return true;
    }

    // fn initialize(&mut self) {
    //     self.vals = Vec::new();
    // }
}

pub fn serialize(val: &str) {
    let mut buff: Vec<Parts> = Vec::new();
    let mut scalar_judger = ScalarJudger::new();

    for i in val.chars() {
        match i {
            '{' => buff.push(Parts::StartDict),
            '}' => buff.push(Parts::EndDict),
            '[' => buff.push(Parts::StartList),
            ']' => buff.push(Parts::EndList),
            ',' => buff.push(Parts::Comma),
            '\n' => {}
            _ => {
                scalar_judger.add_val(&i);
                buff.push(Parts::Others);
            }
        };
    }
    println!("{:?}", scalar_judger);
    scalar_judger.judge();
}
