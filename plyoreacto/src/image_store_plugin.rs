//! Image storing plugin. *Plugin 3*
//! This plugin subscribes to ImageScoredEvent messages and published ImageStoredEvent and
//! ImageDeletedEvent messages.
//!

use std::thread;

use flatbuffers::FlatBufferBuilder;
use rand::Rng;

use crate::events::{bytes_to_event, send_image_deleted_event, send_image_stored_event};

pub fn image_stored_plugin(ctx: &mut zmq::Context) {
    // socket to publish message to
    let mut new_messages = ctx
        .socket(zmq::PUB)
        .expect("Image stored plugin could not create messages socket.");
    new_messages
        .connect("inproc://messages")
        .expect("Image stored plugin could not connect to subscriptions socket");
    println!("Image stored plugin connected to messages socket.");

    // socket to subcribe to events
    let new_events = ctx
        .socket(zmq::SUB)
        .expect("Image stored plugin could not create subscription socket.");
    new_events
        .connect("inproc://events")
        .expect("Image stored plugin could not connect to subscriptions socket");
    // TODO -- subscribe only to image scored events
    let filter = String::new();
    new_events
        .set_subscribe(filter.as_bytes())
        .expect("Image stored plugin could not subscribe to type 1 events on subscription socket");
    println!("Image stored plugin connected to subscription socket.");
    let sync = ctx
        .socket(zmq::REQ)
        .expect("Image stored plugin could not create sync socket.");
    sync.connect("inproc://sync-5002")
        .expect("Image stored plugin could not connect to sync socket.");
    println!("Image stored plugin connected to sync socket.");

    thread::spawn(move || {
        // connect to and send sync message on sync socket
        let msg = "ready";
        sync.send(msg, 0)
            .expect("Image stored plugin could not send sync message");
        println!("Image stored plugin sent sync message.");
        // wait for reply from engine
        let _msg = sync
            .recv_msg(0)
            .expect("Image stored plugin got error trying to receive sync reply");
        println!("Image stored plugin got sync reply, will now block for messages");

        let mut bldr = FlatBufferBuilder::new();
        // for generating random probabilities
        let mut rng = rand::thread_rng();

        // process 5 events
        let mut count = 0;
        while count < 5 {
            let msg_bytes = new_events.recv_bytes(0).expect("Error receiving message");
            let event = bytes_to_event(&msg_bytes).expect("Error getting event");
            // check type of event -- TODO: remove this when subscriptions work
            let event_type = event
                .event_type()
                .variant_name()
                .expect("could not get event type");
            if event_type != "ImageScoredEvent" {
                continue;
            }
            let image_uuid = event
                .event_as_image_scored_event()
                .unwrap()
                .image_uuid()
                .unwrap();
            println!(
                "Image stored plugin got ImageScored event for image {}",
                image_uuid
            );

            // Flip a coin to decide whether to store the image
            // generate a random probability:
            let prob = rng.gen::<f32>();
            // delete the image
            if prob < 0.5 {
                send_image_deleted_event(&mut new_messages, &mut bldr, image_uuid)
                    .expect("could not sent image deleted event");
                println!(
                    "Image stored plugin sent an image deleted event for image {}",
                    image_uuid
                );
            } else {
                send_image_stored_event(&mut new_messages, &mut bldr, image_uuid)
                    .expect("could not sent image deleted event");
                println!(
                    "Image stored plugin sent an image stored event for image {}",
                    image_uuid
                );
            }
            count += 1;
        }
    });
}
