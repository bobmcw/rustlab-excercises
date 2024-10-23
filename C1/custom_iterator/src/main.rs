struct EvenNumbers {
    current: u32,
    max: u32,
}
impl EvenNumbers {
    fn new(max: u32) -> Self {
        EvenNumbers{current: 0,max}
    }
}
impl Iterator for EvenNumbers {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item>{
       self.current += 2; 
       if self.current > self.max{
           None
       }
       else {
          Some(self.current)
       }
    }

}

fn main() {
    let mut even_numbers = EvenNumbers::new(10);
    while let Some(num) = even_numbers.next() {
    println!("{}", num);
    }
    let sum: u32 = EvenNumbers::new(10).sum();
    println!("suma: {}",sum)

}
