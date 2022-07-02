//! New Image plugin. *Plugin 1*
//! This plugin publishes NewImageEvent messages. It does not subscribe to any messages.
//!

use super::events::send_new_image_event;
use flatbuffers::FlatBufferBuilder;
use std::thread;

// try having the engine create the threads, pass a closure that first does the syn and then calls
// the plugin function to the spawn, and
// also do biolerplate code in the engine, then pass the function: new_messages_socket and the new_events

pub fn new_image_plugin(ctx: &mut zmq::Context) {
    // Biolerplate --------------------------------
    let mut new_messages = ctx
        .socket(zmq::PUB)
        .expect("New Image plugin could not create messages socket.");
    new_messages
        .connect("inproc://messages")
        .expect("Image stored plugin could not connect to subscriptions socket");
    println!("New Image plugin connected to messages socket.");
    let sync = ctx
        .socket(zmq::REQ)
        .expect("New Image plugin could not create sync socket.");
    sync.connect("inproc://sync-5000")
        .expect("New Image plugin could not connect to sync socket.");
    println!("New Image plugin connected to sync socket.");
    // Boilerplate -------------------------------

    thread::spawn(move || {
        // connect to and send sync message on sync socket
        let msg = "ready";
        sync.send(msg, 0)
            .expect("New Image plugin could not send sync message");
        println!("New Image plugin sent sync message.");
        // wait for reply from engine
        let _msg = sync
            .recv_msg(0)
            .expect("New Image plugin got error trying to receive sync reply");
        println!("New Image plugin got sync reply, will now block for messages");

        let mut bldr = FlatBufferBuilder::new();

        // send 5 New Image events as fast as we can...
        let mut count = 0;
        while count < 5 {
            let uuid = uuid::Uuid::new_v4().to_string();
            send_new_image_event(
                &mut new_messages,
                &mut bldr,
                &uuid,
                &"png".to_string(),
                &Vec::<u8>::new(),
            )
            .expect("Could not send a new message event");

            println!(
                "(NEW IMAGE -- {}) New Image plugin sent message {:?}",
                uuid, uuid
            );
            count += 1;
        }
    });
}
