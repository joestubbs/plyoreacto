use crate::events_generated::events::{
    Event, EventArgs, EventType, ImageLabelScore, ImageScoredEvent, ImageScoredEventArgs,
};
use flatbuffers::{FlatBufferBuilder, WIPOffset};
use zmq::Socket;

use super::events_generated::events::{
    root_as_event, ImageDeletedEvent, ImageDeletedEventArgs, ImageLabelScoreArgs, ImageStoredEvent,
    ImageStoredEventArgs, NewImageEvent, NewImageEventArgs,
};

pub fn get_event_type_bytes_filter(event_type: &str) -> Result<[u8; 20], String> {
    //TODO -- generate these programmatically
    if event_type == "NewImageEvent" {
        // first bytes of NewImageEvent
        let filter_bytes: [u8; 20] = [12, 0, 0, 0, 8, 0, 12, 0, 7, 0, 8, 0, 8, 0, 0, 0, 0, 0, 0, 1];
        return Ok(filter_bytes);
    } else if event_type == "ImageScoredEvent" {
        let filter_bytes: [u8; 20] = [12, 0, 0, 0, 8, 0, 14, 0, 7, 0, 8, 0, 8, 0, 0, 0, 0, 0, 0, 2];
        return Ok(filter_bytes);
    } else if event_type == "ImageStoredEvent" {
        // first bytes of ImageStoredEvent (TODO)
        let filter_bytes: [u8; 20] = [12, 0, 0, 0, 8, 0, 12, 0, 7, 0, 8, 0, 8, 0, 0, 0, 0, 0, 0, 3];
        return Ok(filter_bytes);
    } else if event_type == "ImageDeletedEvent" {
        // first bytes of ImageDeletedEvent (TODO)
        let filter_bytes: [u8; 20] = [12, 0, 0, 0, 8, 0, 12, 0, 7, 0, 8, 0, 8, 0, 0, 0, 0, 0, 0, 4];
        return Ok(filter_bytes);
    }
    Err("Invalid event_type".to_string())
}

pub fn send_new_image_event(
    msg_socket: &mut Socket,
    bldr: &mut FlatBufferBuilder,
    image_uuid: &str,
    image_format: &str,
    image: &[u8],
) -> Result<(), std::io::Error> {
    bldr.reset();

    let args = NewImageEventArgs {
        message_type: Default::default(),
        image_uuid: Some(bldr.create_string(image_uuid)),
        image_format: Some(bldr.create_string(image_format)),
        image: Some(bldr.create_vector(image)),
    };
    let new_image_event = NewImageEvent::create(bldr, &args);
    let event_args = EventArgs {
        event_type: EventType::NewImageEvent,
        event: Some(new_image_event.as_union_value()),
    };
    let event = Event::create(bldr, &event_args);
    bldr.finish(event, None);

    // send the new_event message over the messages socket
    msg_socket
        .send(bldr.finished_data(), 0)
        .expect("could not send new image event over zmq socket");

    Ok(())
}

pub struct ImageScore {
    pub label: String,
    pub probability: f32,
}

pub fn send_image_scored_event(
    msg_socket: &mut Socket,
    bldr: &mut FlatBufferBuilder,
    image_uuid: &str,
    scores: Vec<ImageScore>,
) -> Result<(), std::io::Error> {
    bldr.reset();

    // create the ImageLabelScoreArgs from scores
    let mut image_label_scores = Vec::<WIPOffset<ImageLabelScore>>::new();
    for score in scores {
        let label = Some(bldr.create_string(&score.label));
        let im_score = ImageLabelScore::create(
            bldr,
            &ImageLabelScoreArgs {
                label,
                probability: score.probability,
            },
        );
        image_label_scores.push(im_score);
    }

    // create a vector of ImageLabelScores
    bldr.create_vector(&image_label_scores);

    // now create the ImageScoredEventArgs
    let args = ImageScoredEventArgs {
        message_type: Default::default(),
        image_uuid: Some(bldr.create_string(image_uuid)),
        scores: Some(bldr.create_vector(&image_label_scores)),
    };

    let image_scored_event = ImageScoredEvent::create(bldr, &args);
    let event_args = EventArgs {
        event_type: EventType::ImageScoredEvent,
        event: Some(image_scored_event.as_union_value()),
    };
    let event = Event::create(bldr, &event_args);
    bldr.finish(event, None);

    // send the new_event message over the messages socket
    msg_socket
        .send(bldr.finished_data(), 0)
        .expect("could not send image scored event over zmq socket");

    Ok(())
}

pub fn send_image_stored_event(
    msg_socket: &mut Socket,
    bldr: &mut FlatBufferBuilder,
    image_uuid: &str,
) -> Result<(), std::io::Error> {
    bldr.reset();

    let args = ImageStoredEventArgs {
        message_type: Default::default(),
        image_uuid: Some(bldr.create_string(image_uuid)),
    };
    let image_stored_event = ImageStoredEvent::create(bldr, &args);

    let event_args = EventArgs {
        event_type: EventType::ImageStoredEvent,
        event: Some(image_stored_event.as_union_value()),
    };
    let event = Event::create(bldr, &event_args);
    bldr.finish(event, None);

    // send the new_event message over the messages socket
    msg_socket
        .send(bldr.finished_data(), 0)
        .expect("could not send image stored event over zmq socket");

    Ok(())
}

pub fn send_image_deleted_event(
    msg_socket: &mut Socket,
    bldr: &mut FlatBufferBuilder,
    image_uuid: &str,
) -> Result<(), std::io::Error> {
    bldr.reset();

    let args = ImageDeletedEventArgs {
        message_type: Default::default(),
        image_uuid: Some(bldr.create_string(image_uuid)),
    };
    let image_deleted_event = ImageDeletedEvent::create(bldr, &args);

    let event_args = EventArgs {
        event_type: EventType::ImageDeletedEvent,
        event: Some(image_deleted_event.as_union_value()),
    };
    let event = Event::create(bldr, &event_args);
    bldr.finish(event, None);
    // send the new_event message over the messages socket
    msg_socket
        .send(bldr.finished_data(), 0)
        .expect("could not send image deleted event over zmq socket");

    Ok(())
}

// pub fn read_next_event(new_events_socket: &Socket) -> std::io::Result<Event<'static>> {
//     let msg_bytes = new_events_socket.recv_bytes(0).expect("Error receiving message");
//     let event = root_as_event(&msg_bytes).expect("could not deserialize bytes");
//     Ok(event)
// }

pub fn bytes_to_event(msg_bytes: &[u8]) -> std::io::Result<Event> {
    let event = root_as_event(msg_bytes).expect("could not deserialize bytes");
    Ok(event)
}

#[cfg(test)]
mod test {
    use crate::events_generated::events::{Event, EventArgs, EventType};
    use flatbuffers::FlatBufferBuilder;

    use super::*;
    use std::fs::{self, OpenOptions};
    use std::io::Write;
    use uuid::Uuid;

    #[test]
    fn test_write_new_image_event_to_file() -> std::io::Result<()> {
        let mut bldr = FlatBufferBuilder::new();

        let image_uuid = Uuid::new_v4().to_string();
        let image_format = "png".to_string();
        let image = Vec::<u8>::new();

        let args = NewImageEventArgs {
            message_type: Default::default(),
            image_uuid: Some(bldr.create_string(&image_uuid)),
            image_format: Some(bldr.create_string(&image_format)),
            image: Some(bldr.create_vector(&image)),
        };
        let new_image_event = NewImageEvent::create(&mut bldr, &args);
        let event_args = EventArgs {
            event_type: EventType::NewImageEvent,
            event: Some(new_image_event.as_union_value()),
        };
        let event = Event::create(&mut bldr, &event_args);
        bldr.finish(event, None);

        // write data to a file
        let mut file = OpenOptions::new()
            .write(true)
            .open("/home/jstubbs/Documents/new_image1.msg")?;
        file.write_all(bldr.finished_data())
            .expect("could not write data");

        Ok(())
    }

    #[test]
    fn test_read_new_image_event_from_file() -> std::io::Result<()> {
        let msg_bytes = fs::read("/home/jstubbs/Documents/new_image1.msg")?;
        let msg = root_as_event(&msg_bytes).expect("could not deserialize bytes");

        let event_type = msg.event_type();
        let variant = event_type
            .variant_name()
            .expect("could not get event variant");
        dbg!(variant);
        assert_eq!("NewImageEvent", variant);

        let new_image_event = msg
            .event_as_new_image_event()
            .expect("could not cast to NewImageEvent");
        let image_uuid = new_image_event
            .image_uuid()
            .expect("could not get image_uuid from event");
        dbg!(image_uuid);

        Ok(())
    }

    #[test]
    fn test_write_image_stored_event_to_file() -> std::io::Result<()> {
        let mut bldr = FlatBufferBuilder::new();
        let image_uuid = Uuid::new_v4();

        let args = ImageStoredEventArgs {
            message_type: Default::default(),
            image_uuid: Some(bldr.create_string(&image_uuid.to_string())),
        };
        let image_stored_event = ImageStoredEvent::create(&mut bldr, &args);

        let event_args = EventArgs {
            event_type: EventType::ImageStoredEvent,
            event: Some(image_stored_event.as_union_value()),
        };
        let event = Event::create(&mut bldr, &event_args);
        bldr.finish(event, None);

        // write data to a file
        let mut file = OpenOptions::new()
            .write(true)
            .open("/home/jstubbs/Documents/image_stored1.msg")?;
        file.write_all(bldr.finished_data())
            .expect("could not write data");

        Ok(())
    }

    #[test]
    fn test_read_image_stored_event_from_file() -> std::io::Result<()> {
        let msg_bytes = fs::read("/home/jstubbs/Documents/image_stored1.msg")?;
        let msg = root_as_event(&msg_bytes).expect("could not deserialize bytes");

        let event_type = msg.event_type();
        let variant = event_type
            .variant_name()
            .expect("could not get event variant");
        dbg!(variant);
        assert_eq!("ImageStoredEvent", variant);

        let stored_event = msg
            .event_as_image_stored_event()
            .expect("could not cast to ImageStoredEvent");
        let image_uuid = stored_event
            .image_uuid()
            .expect("could not get image_uuid from event");
        dbg!(image_uuid);

        Ok(())
    }

    #[test]
    fn test_write_image_scored_event_to_file() -> std::io::Result<()> {
        let mut bldr = FlatBufferBuilder::new();
        let image_uuid = Uuid::new_v4();
        let mut scores = Vec::<ImageScore>::new();
        scores.push(ImageScore {
            label: "labrador".to_string(),
            probability: 0.98,
        });
        scores.push(ImageScore {
            label: "golden retriever".to_string(),
            probability: 0.02,
        });

        let mut image_label_scores = Vec::<WIPOffset<ImageLabelScore>>::new();
        for score in scores {
            let label = Some(bldr.create_string(&score.label));
            let im_score = ImageLabelScore::create(
                &mut bldr,
                &ImageLabelScoreArgs {
                    label: label,
                    probability: score.probability,
                },
            );
            image_label_scores.push(im_score);
        }

        // create a vector of ImageLabelScores
        bldr.create_vector(&image_label_scores);

        // now create the ImageScoredEventArgs
        let args = ImageScoredEventArgs {
            message_type: Default::default(),
            image_uuid: Some(bldr.create_string(&image_uuid.to_string())),
            scores: Some(bldr.create_vector(&image_label_scores)),
        };

        let image_scored_event = ImageScoredEvent::create(&mut bldr, &args);
        let event_args = EventArgs {
            event_type: EventType::ImageScoredEvent,
            event: Some(image_scored_event.as_union_value()),
        };
        let event = Event::create(&mut bldr, &event_args);
        bldr.finish(event, None);

        // write data to a file
        let mut file = OpenOptions::new()
            .write(true)
            .open("/home/jstubbs/Documents/image_scored1.msg")?;
        file.write_all(bldr.finished_data())
            .expect("could not write data");

        Ok(())
    }

    #[test]
    fn test_read_image_scored_event_from_file() -> std::io::Result<()> {
        let msg_bytes = fs::read("/home/jstubbs/Documents/image_scored1.msg")?;
        let msg = root_as_event(&msg_bytes).expect("could not deserialize bytes");

        let event_type = msg.event_type();
        let variant = event_type
            .variant_name()
            .expect("could not get event variant");
        dbg!(variant);
        assert_eq!("ImageScoredEvent", variant);

        let scored_event = msg
            .event_as_image_scored_event()
            .expect("could not cast to ImageScoredEvent");
        let image_uuid = scored_event
            .image_uuid()
            .expect("could not get image_uuid from event");
        dbg!(image_uuid);

        Ok(())
    }

    #[test]
    fn test_write_image_deleted_event_to_file() -> std::io::Result<()> {
        let mut bldr = FlatBufferBuilder::new();
        let image_uuid = Uuid::new_v4();

        let args = ImageDeletedEventArgs {
            message_type: Default::default(),
            image_uuid: Some(bldr.create_string(&image_uuid.to_string())),
        };
        let image_deleted_event = ImageDeletedEvent::create(&mut bldr, &args);

        let event_args = EventArgs {
            event_type: EventType::ImageDeletedEvent,
            event: Some(image_deleted_event.as_union_value()),
        };
        let event = Event::create(&mut bldr, &event_args);
        bldr.finish(event, None);

        // write data to a file
        let mut file = OpenOptions::new()
            .write(true)
            .open("/home/jstubbs/Documents/image_deleted1.msg")?;
        file.write_all(bldr.finished_data())
            .expect("could not write data");

        Ok(())
    }

    #[test]
    fn test_read_image_deleted_event_from_file() -> std::io::Result<()> {
        let msg_bytes = fs::read("/home/jstubbs/Documents/image_deleted1.msg")?;
        let msg = root_as_event(&msg_bytes).expect("could not deserialize bytes");

        let event_type = msg.event_type();
        let variant = event_type
            .variant_name()
            .expect("could not get event variant");
        dbg!(variant);
        assert_eq!("ImageDeletedEvent", variant);

        let deleted_event = msg
            .event_as_image_deleted_event()
            .expect("could not cast to ImageDeletedEvent");
        let image_uuid = deleted_event
            .image_uuid()
            .expect("could not get image_uuid from event");
        dbg!(image_uuid);

        Ok(())
    }
}
