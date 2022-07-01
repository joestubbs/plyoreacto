//! Image scoring plugin. *Plugin 2*
//! This plugin subscribes to NewImageEvent messages and published ImageScoredEvent messages.
//!

use crate::events::bytes_to_event;
use flatbuffers::FlatBufferBuilder;
use rand::Rng;
use std::thread;

use super::events::{send_image_scored_event, ImageScore};

pub fn image_scored_plugin(ctx: &mut zmq::Context) {
    // socket to publish message to
    let mut new_messages = ctx
        .socket(zmq::PUB)
        .expect("Image Scored plugin could not create messages socket.");
    new_messages
        .connect("inproc://messages")
        .expect("Image stored plugin could not connect to subscriptions socket");
    println!("Image Scored plugin connected to messages socket.");

    // socket to subcribe to events
    let new_events = ctx
        .socket(zmq::SUB)
        .expect("Image scored plugin could not create subscription socket.");
    new_events
        .connect("inproc://events")
        .expect("Image scored plugin could not connect to subscriptions socket");
    // TODO -- subscribe only to new image events
    let filter = String::new();
    new_events
        .set_subscribe(filter.as_bytes())
        .expect("Image scored plugin could not subscribe to type 1 events on subscription socket");
    println!("Image scored plugin connected to subscription socket.");
    let sync = ctx
        .socket(zmq::REQ)
        .expect("Image scored plugin could not create sync socket.");
    sync.connect("inproc://sync-5001")
        .expect("Image scored plugin could not connect to sync socket.");
    println!("Image scored plugin connected to sync socket.");

    thread::spawn(move || {
        // connect to and send sync message on sync socket
        let msg = "ready";
        sync.send(msg, 0)
            .expect("Image scored plugin could not send sync message");
        println!("Image scored plugin sent sync message.");
        // wait for reply from engine
        let _msg = sync
            .recv_msg(0)
            .expect("Image scored plugin got error trying to receive sync reply");
        println!("Image scored plugin got sync reply, will now block for messages");

        let mut bldr = FlatBufferBuilder::new();

        // process 5 new image events
        let mut count = 0;
        // for generating random probabilities
        let mut rng = rand::thread_rng();

        while count < 5 {
            let msg_bytes = new_events.recv_bytes(0).expect("Error receiving message");
            let event = bytes_to_event(&msg_bytes).expect("Error getting event");
            // let event = read_next_event(&new_events).expect("could not get next event.");

            // check type of event -- TODO: remove this when subscriptions work
            let event_type = event
                .event_type()
                .variant_name()
                .expect("could not get event type");
            if event_type != "NewImageEvent" {
                continue;
            }
            let image_uuid = event
                .event_as_new_image_event()
                .unwrap()
                .image_uuid()
                .unwrap();
            println!(
                "Image scored plugin got New Image event for image {}",
                image_uuid
            );
            // generate an image scored event
            // generate a random probability:
            let prob = rng.gen::<f32>();
            let mut scores = Vec::<ImageScore>::new();
            scores.push(ImageScore {
                label: "laborador".to_string(),
                probability: prob,
            });
            send_image_scored_event(&mut new_messages, &mut bldr, &image_uuid, scores)
                .expect("Could not send image scored event");
            count += 1;
            println!(
                "Image scored plugin sent Image Scored event for image: {}; prob: {}",
                image_uuid, prob
            );
        }
    });
}
