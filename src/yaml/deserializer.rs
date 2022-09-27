use crate::json::value::Value;
use crate::json::part::Part;

pub struct Deserializer {}
impl Deserializer {
    pub fn new() -> Self {
        Deserializer{}
    }

    /**
     * @todo refactor
     * - dict/arrayが始まったときにキーを表示する
     * - dict/arrayが空だっとときに {} を表示する
     */
    pub fn deserialize(&mut self, values: Vec<Value>) -> String {
        let mut out :String = String::from("");
        for value in values {
            let path = value.path;
            let indicators = path.indicators.clone();
            let indicators_len = indicators.len();
            let mut spaces = String::from("");
            // show key if value.part == Part::StartDict && not root object
            // append [] or {} if value is empty
            for (i, indicator) in indicators.iter().enumerate() {
                if indicator.indicate == *"dict" {
                    if i < indicators_len - 1 {
                        let mut k = i;
                        let mut show_key = true;
                        while k < indicators_len - 1 {
                            let next = indicators[k+1].clone();
                            if next.count > 1 {
                                show_key = false;
                                break;
                            }
                            k += 1;
                        }
                        if show_key {
                            let mut key = path.value[i].clone();
                            key = String::from(&key[1..]) + ": ";
                            out += &format!("{}{}\n", spaces, key);
                            spaces += "  ";
                        } else {
                            spaces += "  ";
                        }
                    } else {
                        let mut key = path.value.last().unwrap().clone();
                        key = String::from(&key[1..]) + ": ";
                        spaces += &key;
                    }
                }
                if indicator.indicate == *"list" {
                    // print!("- ");
                    if i < indicators_len - 1 {
                        let next = indicators[i+1].clone();
                        if next.count == 1 {
                            spaces += "- ";
                        } else {
                            spaces += "  ";
                        }
                    } else {
                        spaces += "- ";
                    }
                } 
            }
            out += &format!("{}{}\n", spaces, value.part);
        }
        out
    }

    // pub fn deserialize(&mut self, values: Vec<Value>) -> String {
    //     let mut out :String = String::from("");
    //     let mut spaces: usize = 0;
    //     for value in values {
    //         let path = value.path.clone();
    //         let part = value.part.clone();
    //         // println!("{:?}", value);

    //         if let Some(indicator) = path.indicators.last() {
    //             if indicator.indicate == *"dict" {
    //                 if part == Part::StartDict {
    //                     // not root
    //                     if path.indicators.len() != 1 {
    //                         let mut key = path.value.get(path.value.len() - 2).unwrap().clone();
    //                         key = String::from(&key[1..]);
    //                         out += &format!("{}{}: \n", " ".repeat(spaces), key);
    //                         spaces += 2;
    //                     }
    //                 } else {
    //                     let mut key = path.value.last().unwrap().clone();
    //                     key = String::from(&key[1..]);
    //                     out += &format!("{}{}: {}\n", " ".repeat(spaces), key, part);
    //                 }
    //                 // if part == Part::EndDict {
    //                 //     spaces -= 2;
    //                 // }
    //             }
    //             // @todo 配列の中にオブジェクトがあるときを考慮する
    //             if indicator.indicate == *"list" {
    //                 if part == Part::StartList {
    //                     // not root
    //                     if path.indicators.len() != 1 {
    //                         let mut key = path.value.get(path.value.len() - 2).unwrap().clone();
    //                         key = String::from(&key[1..]);
    //                         out += &format!("{}{}: \n", " ".repeat(spaces), key);
    //                     }
    //                 } else {
    //                     out += &format!("{}- {}\n", " ".repeat(spaces), part);
    //                 }
    //             }
    //         }
    //     }
    //     out
    // }

    pub fn print_debug(&mut self, values: Vec<Value>) {
        for mut value in values {
            println!("{}\t {:?}", value.path.to_string(), value.part);
        }
    }
}