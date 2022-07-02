use std::thread;

mod event_engine;
mod events;

#[allow(dead_code, unused_imports)]
mod events_generated;
mod image_score_plugin;
mod image_store_plugin;
mod new_image_plugin;

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
    println!("plugin c connected to subscription socket.");
    let sync = ctx
        .socket(zmq::REQ)
        .expect("Plugin c could not create sync socket.");
    sync.connect("inproc://sync-5002")
        .expect("Plugin c could not connect to sync socket.");
    println!("plugin c connected to sync socket.");

    thread::spawn(move || {
        // connect to and send sync message on sync socket
        let msg = "ready";
        sync.send(msg, 0)
            .expect("Plugin c could not send sync message");
        println!("Plugin c sent sync message.");
        // wait for reply from engine
        let _msg = sync
            .recv_msg(0)
            .expect("Plugin c got error trying to receive sync reply");
        println!("Plugin c got sync reply, will now block for messages");

        // process 5 events
        let mut count = 0;
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

    // * --------------------------------------------
    // *
    // * EXAMPLE ENGINE
    // *
    // * Comment the line below to run the demo code
    // * -----------------------------------
    event_engine::event_engine().expect("Error from engine");

    // *---------------------------------------------
    // *
    // * DEMO CODE
    // *
    // * --------------------------------------------
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
    // let filter = "type";
    let filter = String::new();
    incoming
        .set_subscribe(filter.as_bytes())
        .expect("Engine could not subscribe to all events on incoming socket");

    // start plugin c
    plugin_c(&mut context);

    let total_subscribers = 3;
    let mut sync_sockets = Vec::<zmq::Socket>::new();

    // wait for all plugins to sync
    let mut ready_subscribers = 0;
    // the approach below assumes each plugin has been assigned a specific port which implies a degree of
    // coordination between engine and plugins. we could send all sync messages on the same socket/port
    while ready_subscribers < total_subscribers {
        // each subscriber gets its own port
        let port = 5000 + ready_subscribers;
        // synchronization sockets --
        let sync = context
            .socket(zmq::REP)
            .expect("Engine could not create synchronization socket");
        let tcp_addr = format!("tcp://*:{}", port);
        let inproc_addr = format!("inproc://sync-{}", port);
        sync.bind(&tcp_addr)
            .expect("Engine could not bind sync TCP socket.");
        println!("Engine bound to sync TCP socket on port: {}", &port);
        sync.bind(&inproc_addr)
            .expect("Engine could not bind sync inproc socket.");
        println!("Engine bound to sync inproc socket: {}", &inproc_addr);
        // receive message from plugin
        let _msg = sync
            .recv_msg(0)
            .expect("Engine got error receiving sync message");
        println!("Engine got sync message on sync socket {}", &port);
        sync_sockets.push(sync);
        ready_subscribers += 1;
    }
    // send a reply to all plugins
    let mut msg_sent = 0;
    while msg_sent < total_subscribers {
        let reply = "ok";
        let sync = sync_sockets.pop().expect("Could not get sync socket");
        println!("Engine sending reply message to {}", &msg_sent);
        sync.send(reply, 0)
            .expect("Engine got error trying to send sync reply.");
        msg_sent += 1;
    }

    // start the zmq proxy
    println!("Engine starting the proxy...");
    let _result = zmq::proxy(&incoming, &outgoing)
        .expect("Engine got error running proxy; socket was closed?");
}
