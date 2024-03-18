use std::io;
const health:i32=100;
const mana:i32=100;
const stamina:i32=100
const damage:i32=15;
fn main() {
    let mut health = 100;
    crysis(3, &mut health);
    println!("Health after crysis: {}", health);
}

fn crysis(sec:i32,health:&mut i32){
    for i in 0..sec {
        *health-=damage;
    };
}
