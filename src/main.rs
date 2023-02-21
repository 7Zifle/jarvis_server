use zmq;

fn main() {
    let context = zmq::Context::new();
    let responder = context.socket(zmq::ROUTER).unwrap();

    assert!(responder.bind("tcp://*:43599").is_ok());

    println!("Starting loop");

    loop {
        let response = responder.recv_string(0).unwrap();
        match response {
            Ok(str) => println!("Response: {}", str),
            Err(_e) => eprintln!("Error: "),
        }
    }
}
