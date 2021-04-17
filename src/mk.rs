use std::fmt;
use rand::prelude::*;
use std::thread;
use std::marker::Send;


// type TwoDimensions = < Send + Fn(f32,f32) ->bool>;
// type Thunk = Box<dyn Fn() + Send + 'static>;

pub struct MontoCarlo <F: 'static +  Send + Fn(f32,f32) -> bool> {
    count : u32,
    hits : u32,
    func : F,
    max_iter : u32 
}

impl <F: Send + Fn(f32,f32) -> bool> std::clone::Clone for MontoCarlo<F> {
    fn clone(&self) -> Self {
        *self
    }
}

// impl <F: Send + Fn(f32,f32) -> bool> Copy for MontoCarlo<F> {}

impl <F: 'static + Send + Fn(f32,f32) -> bool> fmt::Debug for MontoCarlo<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MonteCarlo")
         .field("count", &self.count)
         .field("hit", &self.hits)
         .finish()
    }
}

impl <F: 'static + Send + Fn(f32,f32) -> bool> MontoCarlo <F> {

    // fn new(f : &dyn Fn(f32,f32) -> bool) -> MontoCarlo {
    // pub fn new(f : &dyn Fn(i32,i32) -> bool) -> MontoCarlo {
    pub fn new(f : F, i : u32) -> Self {
        return MontoCarlo{
            count:0, 
            hits : 0,
            func: f,
            max_iter : i
        }
    }

    pub fn eval(& mut self,x : f32, y : f32) {
        self.count += 1;
        self.hits += (&self.func)(x,y) as u32;
    }

    pub fn hits(self) -> f32 {
        return self.hits as f32 / self.count as f32
    }

    pub fn sample(& mut self) {
        let mut rng = thread_rng();
        let x: f32 = rng.gen();
        let y: f32 = rng.gen();

        self.eval(x,y)
    }

    pub fn simulate(& mut self) {
        for _i in 1..self.max_iter {
            self.sample()
        }
    }

    // pub fn simulate_parallel(& mut self, num_threads : u32) {
    //     let thread_max_iter = self.max_iter / num_threads;

    //     let mut children = vec![];

    //     for _i in 1..num_threads {
    //         // let mut c = self.clone();
    //         let mut self_clone = self.clone();
    //         children.push(thread::spawn( move || {
    //             for _j in 1..thread_max_iter {
    //                 self_clone.sample();
    //             }
    //         }
    //         ))
    //     }
    // }

}

