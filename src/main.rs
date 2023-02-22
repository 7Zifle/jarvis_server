use chrono::prelude::*;
use zmq;
fn main() {
    println!("Starting jarvis");
    let context = zmq::Context::new();
    let responder = context.socket(zmq::ROUTER).unwrap();

    assert!(responder.bind("tcp://*:43599").is_ok());

    println!("Starting loop");

    loop {
        let msg = responder.recv_multipart(0).expect("Unable to view message");
        println!("received msg");
        let id = format!("ID: {:X?}", msg[0].clone());
        println!("id: {}", id);
        // we will ignore msg[1] since its padding
        let msg = String::from_utf8(msg[2].clone()).expect("Unable to convert msg[2] to string");
        let current_time: DateTime<Local> = Local::now();
        if current_time.hour() >= 6 && current_time.hour() < 22 {
            // start playing music
        }
        println!("msg: {}", msg);
    }
}
#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    #[test]
    fn should_be_playing() {}
}
