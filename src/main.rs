mod mk;

fn circle(x : f32, y:f32) -> bool {
    return (x.powf(2.0) + y.powf(2.0)) <= 1.0
}

fn diamond(x : f32, y:f32) -> bool {
    return x + y < 1.0
}

fn main() {

    let mut c  = mk::MontoCarlo::new(&'circle, 1000000);
    c.simulate();
    println!("{:?}",c);
    println!("Hits {:.8}", c.hits()*4.0);

    let mut d = mk::MontoCarlo::new(diamond, 1000000);
    d.simulate();
    println!("{:?}",d);
    println!("Hits {:.8}", d.hits()*4.0);        

}
