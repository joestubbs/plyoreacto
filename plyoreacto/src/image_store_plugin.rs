//! Image storing plugin. *Plugin 3*
//! This plugin subscribes to ImageScoredEvent messages and published ImageStoredEvent and
//! ImageDeletedEvent messages.
//!

use flatbuffers::FlatBufferBuilder;
use zmq::Socket;

use crate::events::{bytes_to_event, send_image_deleted_event, send_image_stored_event};

pub fn start(
    pub_socket: &mut Socket,
    sub_socket: &mut Socket,
    bldr: &mut FlatBufferBuilder,
) -> std::io::Result<()> {
    // process 5 events
    let mut count = 0;
    while count < 5 {
        let msg_bytes = sub_socket.recv_bytes(0).expect("Error receiving message");
        let event = bytes_to_event(&msg_bytes).expect("Error getting event");
        // check type of event -- TODO: remove this when subscriptions work
        let event_type = event
            .event_type()
            .variant_name()
            .expect("could not get event type");
        if event_type != "ImageScoredEvent" {
            println!("******** Image store plugin got unexpected message!!!**********");
            continue;
        }

        let image_scored_event = event
            .event_as_image_scored_event()
            .expect("could not cast event to ImageScoredEvent");
        let image_uuid = image_scored_event.image_uuid().unwrap();
        println!(
            "Image stored plugin got ImageScored event for image {}",
            image_uuid
        );
        // If the probability of the image containing a laborador is > 0.5, we keep the image
        let scores = image_scored_event
            .scores()
            .expect("could not get image scores");
        for score in scores {
            if score.label().expect("could not get score label") == "labrador" {
                // found the labrador score, check the probability
                if score.probability() < 0.5 {
                    send_image_deleted_event(pub_socket, bldr, image_uuid)
                        .expect("could not sent image deleted event");
                    println!(
                        "(IMAGE DELETED -- {}) Image stored plugin sent an image deleted event for image {}", image_uuid,
                        image_uuid
                    );
                } else {
                    send_image_stored_event(pub_socket, bldr, image_uuid)
                        .expect("could not sent image deleted event");
                    println!(
                        "(IMAGE STORED -- {}) Image stored plugin sent an image stored event for image {}", image_uuid, 
                        image_uuid
                    );
                }
            }
        }
        count += 1;
    }

    Ok(())
}
