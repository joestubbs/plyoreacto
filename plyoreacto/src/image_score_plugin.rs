//! Image scoring plugin. *Plugin 2*
//! This plugin subscribes to NewImageEvent messages and published ImageScoredEvent messages.
//!

use crate::events::bytes_to_event;
use flatbuffers::FlatBufferBuilder;
use rand::Rng;
use zmq::Socket;

use super::events::{send_image_scored_event, ImageScore};

pub fn start(
    pub_socket: &mut Socket,
    sub_socket: &mut Socket,
    bldr: &mut FlatBufferBuilder,
) -> std::io::Result<()> {
    // process 5 new image events
    let mut count = 0;
    // for generating random probabilities
    let mut rng = rand::thread_rng();

    while count < 5 {
        let msg_bytes = sub_socket.recv_bytes(0).expect("Error receiving message");
        let event = bytes_to_event(&msg_bytes).expect("Error getting event");
        // check type of event -- TODO: remove this when subscriptions work
        let event_type = event
            .event_type()
            .variant_name()
            .expect("could not get event type");
        if event_type != "NewImageEvent" {
            println!("*********** Image score plugin got unexpected message!!! ***********");
            println!("Message variant: {}", event_type);
            println!("Message bytes: {:?}", &msg_bytes);
            println!("**********                                               ************");
            continue;
        };

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
        let scores = vec![ImageScore {
            label: "labrador".to_string(),
            probability: prob,
        }];
        send_image_scored_event(pub_socket, bldr, image_uuid, scores)
            .expect("Could not send image scored event");
        count += 1;
        println!(
            "(IMAGE SCORED -- {}) Image scored plugin sent Image Scored event for image: {}; prob: {}", image_uuid, 
            image_uuid, prob
        );
    }
    Ok(())
}
