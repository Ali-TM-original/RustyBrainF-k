use std::char;
use std::collections::HashMap;
#[allow(dead_code)]
pub struct Interpret{
    source:Vec<char>,
    pointer:usize,
    result:usize,
    cell:usize,
}

/// 8 bit bf interpreter
impl Interpret{
    pub fn new(source:String)->Interpret{
        Interpret{source:source.chars().collect(), pointer:0, result:0, cell:0}
    }

    pub fn inter(&mut self)->Vec<i32>{
        let mut numbers: Vec<i32> = vec![0,0,0,0,0,0,0,0];
        let loop_stack = self.getloopstack();
        loop {
            if !(self.source.len() > self.pointer){
                break
            }
            self.skipwhitespace();
            let ch:char = self.get_chars();
            match ch{
                '+'=>{
                    numbers[self.cell] += 1;
                    if numbers[self.cell]>255{
                        numbers[self.cell]=0;
                    }
                    self.pointer+=1;
                },
                '-'=>{
                    numbers[self.cell]-=1;
                    if numbers[self.cell]<0{
                        numbers[self.cell]=255;
                    }
                    self.pointer+=1;
                }
                '>'=>{
                    if !self.cell >7{
                    self.cell+=1; self.pointer+=1}else{
                        panic!("Syntax Error unable to increment pointer")
                    }
                },
                '<'=>{
                    if self.cell != 0{
                    self.cell-=1;self.pointer+=1}else{panic!("Syntax Error unable to decrement pointer")}
                },
                '['=>{ // current cell value is 0 skip past the loop
                    if numbers[self.cell]==0{
                        let _bor = (self.pointer.clone()) as i32;
                        let x = loop_stack.get_key_value(&_bor);
                        //println!("here with {:?} inside [", loop_stack);
                        self.pointer = x.unwrap().1.to_owned();
                        println!("{:?}", x.unwrap().1);
                    }else{
                        self.pointer+=1;
                    }
                },
                ']'=>{ // Current cell value not zero return to first bracket
                    if numbers[self.cell]!=0{
                        let _bor = (self.pointer.clone()) as i32;
                        //println!("here with {:?} in ]", loop_stack);
                        //println!("{}", self.pointer);
                        for (key, value) in loop_stack.iter(){
                            if value == &self.pointer{
                                self.pointer = key.to_owned() as usize
                            }
                        }
                    }else{
                        self.pointer+=1
                    };
                },
                '.'=>{
                    // output statement
                    // Convert everything into ascii characters 
                    println!("{:?}",char::from_u32(numbers[self.cell] as u32).unwrap());
                    self.pointer+=1;
                },
                ','=>{
                    self.pointer +=1;
                }
                _=>{
                    break
                }
            }
        }
        numbers
    }

    fn get_chars(&mut self)->char{
        *self.source.get(self.pointer).unwrap()
    }

    pub fn getloopstack(&mut self)->HashMap<i32, usize>{
        let mut hash_table = HashMap::new();
        let mut loop_stack:Vec<i32> = Vec::new();
        for (index, char_) in self.source.iter().enumerate() {
            if char_.to_owned() == '['{
                loop_stack.push(index as i32);
            }else if char_.to_owned() == ']'{
                let loop_begining_index = loop_stack.pop().unwrap();
                //hash_table.insert(index+1, loop_begining_index);
                //hash_table.insert(index+1,loop_begining_index);
                hash_table.insert(loop_begining_index, index);
            }
        }
        hash_table

    }

    fn skipwhitespace(&mut self){
        //let char = self.char;
        while self.get_chars().is_whitespace(){
            self.pointer+=1;
        }
    }
}

