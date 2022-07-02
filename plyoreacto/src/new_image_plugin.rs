//! New Image plugin. *Plugin 1*
//! This plugin publishes NewImageEvent messages. It does not subscribe to any messages.
//!

use super::events::send_new_image_event;
use flatbuffers::FlatBufferBuilder;
use zmq::Socket;

pub fn start(
    pub_socket: &mut Socket,
    _sub_socket: &mut Socket,
    bldr: &mut FlatBufferBuilder,
) -> std::io::Result<()> {
    // send 5 New Image events as fast as we can...
    let mut count = 0;
    while count < 5 {
        let uuid = uuid::Uuid::new_v4().to_string();
        send_new_image_event(
            pub_socket,
            bldr,
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
    Ok(())
}
