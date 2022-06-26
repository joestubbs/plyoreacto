use std::thread;

fn plugin_c(ctx: &mut zmq::Context) {
    let new_events = ctx
        .socket(zmq::SUB)
        .expect("Plugin c could not create subscription socket.");
    new_events
        .connect("inproc://events")
        .expect("Plugin c could not connect to subscriptions socket");
    // subscribe to type 1 events
    let filter = "type:1";
    new_events
        .set_subscribe(filter.as_bytes())
        .expect("Plugin c could not subscribe to type 1 events on subscription socket");
    println!("plugin c connected to subscription socket");

    thread::spawn(move || {
        let mut count = 0;
        // process 5 events
        while count < 5 {
            let msg = new_events.recv_msg(0).expect("Error receiving message");
            let msgs = msg.as_str().expect("couln't convert msg to string");
            println!("plugin c got message {:?}", msgs);
            count += 1;
        }
    });
}

fn main() {
    println!("Starting main engine");
    let mut context = zmq::Context::new();

    // socket used by the engine for outgoing events
    let outgoing = context
        .socket(zmq::PUB)
        .expect("Engine could not create outgoing socket");
    outgoing
        .bind("tcp://*:5560")
        .expect("Engine could not bind outgoing TCP socket");
    outgoing
        .bind("inproc://events")
        .expect("Engine could not bind outgoing inproc socket");

    // socket used by the engine for incoming events
    let incoming = context
        .socket(zmq::SUB)
        .expect("Engine could not create incoming socket");
    incoming
        .bind("tcp://*:5559")
        .expect("Engine could not bind incoming TCP socket");
    incoming
        .bind("inproc://messages")
        .expect("Engine could not bind incoming inproc socket");

    // subscribe to all events
    let filter = "type";
    incoming
        .set_subscribe(filter.as_bytes())
        .expect("Engine could not subscribe to all events on incoming socket");

    // start plugin c
    plugin_c(&mut context);

    // start the zmq proxy
    println!("Engine starting the proxy...");
    let _result = zmq::proxy(&incoming, &outgoing)
        .expect("Engine got error running proxy; socket was closed?");
}
