fn gen_primes(end: usize)-> Vec<u32>{
    let mut primes = vec![true;end];
    primes[0] = false;
    primes[1] = false;
    let mut j:usize;
    let sqrt: usize = (end as f64).sqrt() as usize;
    for i in 2..=(sqrt){
        if primes[i]{
            j = i*i;
            while j < end{
                primes[j] = false;
                j = j+i;
            }
        }
    }
    let mut nums: Vec<u32> = vec![];
    for (i,val) in primes.iter().enumerate(){
        if *val {
            nums.push(i as u32);
        }
    }
    nums
}
struct PrimeNumbers {
    primes: Vec<u32>,
    current: u32,
}
impl PrimeNumbers{
    fn new(max:u32)-> Self{
        PrimeNumbers{
            primes: gen_primes(max as usize),
            current: 0,
        }
    }
}
impl Iterator for PrimeNumbers {
   type Item = u32;
   fn next(&mut self) -> Option<Self::Item> {
    let current: usize= self.current as usize;
    self.current += 1;
    let res = self.primes.get(current);
    match res {
        None => None,
        Some(value) => Some(*value),
    }
    }
}
fn main() {
    let mut primes: PrimeNumbers = PrimeNumbers::new(100);
    while let Some(num) = primes.next() {
       println!("{}",num) 
    }

    }