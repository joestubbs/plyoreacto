use crate::events_generated::events::{
    Event, EventArgs, EventType, ImageLabelScore, ImageScoredEvent, ImageScoredEventArgs,
};
use flatbuffers::{FlatBufferBuilder, WIPOffset};
use std::collections::HashSet;
use uuid::Uuid;
use zmq::Socket;

use super::events_generated::events::{
    root_as_event, ImageDeletedEvent, ImageDeletedEventArgs, ImageLabelScoreArgs, ImageStoredEvent,
    ImageStoredEventArgs, NewImageEvent, NewImageEventArgs,
};

pub struct Ex {
    name: String,
    // name: [&'a i32],
    // image: [i32],
    // image: [u8],
    // image: String,
}

pub fn example() -> Vec<Ex> {
    let x = String::from("Joe");
    let y = String::from("Rich");
    let e1 = Ex { name: x };
    let e2 = Ex { name: y };
    let mut result = Vec::<Ex>::new();
    result.insert(0, e1);

    use rand::Rng;
    let mut rng = rand::thread_rng();
    let prob = rng.gen::<f32>();
    if prob < 0.5 {
        result.insert(0, e2);
    }
    result
}

pub fn ex2() -> std::io::Result<Vec<u8>> {
    let mut bldr_1 = FlatBufferBuilder::new();
    let image_uuid = Uuid::new_v4().to_string();
    let image_stored_msg = make_image_stored_msg2(&mut bldr_1, &image_uuid).unwrap();

    Ok(image_stored_msg)
}

pub fn compute_event_type_bytes_filters() -> std::io::Result<()> {
    let mut bldr_1 = FlatBufferBuilder::new();
    let mut bldr_2 = FlatBufferBuilder::new();
    let mut bldr_3 = FlatBufferBuilder::new();
    let mut bldr_4 = FlatBufferBuilder::new();

    let image_uuid = Uuid::new_v4().to_string();
    let image_format = "png".to_string();
    let image = Vec::<u8>::new();

    let new_image_msg =
        make_new_image_msg(&mut bldr_1, &image_uuid, &image_format, &image).unwrap();

    let mut scores = Vec::<ImageScore>::new();
    scores.push(ImageScore {
        label: "labrador".to_string(),
        probability: 0.98,
    });
    let image_scored_msg = make_image_scored_msg(&mut bldr_2, &image_uuid, scores).unwrap();
    let image_stored_msg = make_image_stored_msg(&mut bldr_3, &image_uuid).unwrap();
    let image_deleted_msg = make_image_deleted_msg(&mut bldr_4, &image_uuid).unwrap();

    let mut end_position = 0;

    for i in 0..new_image_msg.len() {
        // create a set of the bytes at position i
        let mut bytes_seen = HashSet::<u8>::new();
        // add the byte at position i to the set for every message
        bytes_seen.insert(new_image_msg[i]);
        bytes_seen.insert(image_scored_msg[i]);
        bytes_seen.insert(image_stored_msg[i]);
        bytes_seen.insert(image_deleted_msg[i]);
        // if all the bytes at position i were unique, then we have found the end
        // position and we can break out of the loop;
        if bytes_seen.len() == 4 {
            end_position = i;
            break;
        }
    }

    let new_message_filter = &new_image_msg[0..end_position + 1];
    let image_scored_filter = &image_scored_msg[0..end_position + 1];
    let image_stored_filter = &image_stored_msg[0..end_position + 1];
    let image_deleted_filter = &image_deleted_msg[0..end_position + 1];

    println!("NewImageMsg filter: {:?}", new_message_filter);
    println!("ImageScoredMsg filter: {:?}", image_scored_filter);
    println!("ImageStoredMsg filter: {:?}", image_stored_filter);
    println!("ImageDeletedMsg filter: {:?}", image_deleted_filter);

    Ok(())
}

pub fn get_event_type_bytes_filter(event_type: &str) -> Result<[u8; 20], String> {
    //TODO -- generate these programmatically
    if event_type == "NewImageEvent" {
        // first bytes of NewImageEvent
        let filter_bytes: [u8; 20] = [12, 0, 0, 0, 8, 0, 14, 0, 7, 0, 8, 0, 8, 0, 0, 0, 0, 0, 0, 1];
        return Ok(filter_bytes);
    } else if event_type == "ImageScoredEvent" {
        let filter_bytes: [u8; 20] = [12, 0, 0, 0, 8, 0, 12, 0, 7, 0, 8, 0, 8, 0, 0, 0, 0, 0, 0, 2];
        return Ok(filter_bytes);
    } else if event_type == "ImageStoredEvent" {
        // first bytes of ImageStoredEvent (TODO)
        let filter_bytes: [u8; 20] = [12, 0, 0, 0, 8, 0, 14, 0, 7, 0, 8, 0, 8, 0, 0, 0, 0, 0, 0, 3];
        return Ok(filter_bytes);
    } else if event_type == "ImageDeletedEvent" {
        // first bytes of ImageDeletedEvent (TODO)
        let filter_bytes: [u8; 20] = [12, 0, 0, 0, 8, 0, 14, 0, 7, 0, 8, 0, 8, 0, 0, 0, 0, 0, 0, 4];
        return Ok(filter_bytes);
    }
    Err("Invalid event_type".to_string())
}

pub fn make_new_image_msg<'a>(
    bldr: &'a mut FlatBufferBuilder,
    image_uuid: &'a str,
    image_format: &'a str,
    image: &'a [u8],
) -> Result<&'a [u8], std::io::Error> {
    bldr.reset();
    let args = NewImageEventArgs {
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

    Ok(bldr.finished_data())
}

// Avoids liftimes by returning an owned data structure, Vec<u8>
// This function makes a copy of the message data using to_vec, so memory consumption
// is likely to be greater. 
pub fn make_new_image_msg_copy(
    bldr: &mut FlatBufferBuilder,
    image_uuid: & str,
    image_format: & str,
    image: & [u8],
) -> std::io::Result<Vec<u8>> {
    bldr.reset();
    let args = NewImageEventArgs {
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

    // to_vec makes a copy of the data.
    Ok(bldr.finished_data().to_vec())
}


pub fn send_new_image_event(
    msg_socket: &mut Socket,
    bldr: &mut FlatBufferBuilder,
    image_uuid: &str,
    image_format: &str,
    image: &[u8],
) -> Result<(), std::io::Error> {
    bldr.reset();

    // make the new image event message
    let data = make_new_image_msg(bldr, image_uuid, image_format, image).unwrap();
    // send the new_event message over the messages socket
    msg_socket
        .send(data, 0)
        .expect("could not send new image event over zmq socket");
    Ok(())
}

pub struct ImageScore {
    pub label: String,
    pub probability: f32,
}

pub fn make_image_scored_msg<'a>(
    bldr: &'a mut FlatBufferBuilder,
    image_uuid: &'a str,
    scores: Vec<ImageScore>,
) -> Result<&'a [u8], std::io::Error> {
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

    let data = bldr.finished_data();

    Ok(data)
}

pub fn send_image_scored_event(
    msg_socket: &mut Socket,
    bldr: &mut FlatBufferBuilder,
    image_uuid: &str,
    scores: Vec<ImageScore>,
) -> Result<(), std::io::Error> {
    let data = make_image_scored_msg(bldr, image_uuid, scores).unwrap();

    // send the new_event message over the messages socket
    msg_socket
        .send(data, 0)
        .expect("could not send image scored event over zmq socket");

    Ok(())
}

pub fn make_image_stored_msg<'a>(
    bldr: &'a mut FlatBufferBuilder,
    image_uuid: &'a str,
) -> Result<&'a [u8], std::io::Error> {
    bldr.reset();

    let args = ImageStoredEventArgs {
        image_uuid: Some(bldr.create_string(image_uuid)),
    };
    let image_stored_event = ImageStoredEvent::create(bldr, &args);

    let event_args = EventArgs {
        event_type: EventType::ImageStoredEvent,
        event: Some(image_stored_event.as_union_value()),
    };
    let event = Event::create(bldr, &event_args);
    bldr.finish(event, None);

    Ok(bldr.finished_data())
}

pub fn make_image_stored_msg2(
    bldr: &mut FlatBufferBuilder,
    image_uuid: &str,
) -> std::io::Result<Vec<u8>> {
    bldr.reset();

    let args = ImageStoredEventArgs {
        image_uuid: Some(bldr.create_string(image_uuid)),
    };
    let image_stored_event = ImageStoredEvent::create(bldr, &args);

    let event_args = EventArgs {
        event_type: EventType::ImageStoredEvent,
        event: Some(image_stored_event.as_union_value()),
    };
    let event = Event::create(bldr, &event_args);
    bldr.finish(event, None);

    Ok(bldr.finished_data().to_vec())
}

pub fn send_image_stored_event(
    msg_socket: &mut Socket,
    bldr: &mut FlatBufferBuilder,
    image_uuid: &str,
) -> Result<(), std::io::Error> {
    // make the image stored message
    let data = make_image_stored_msg(bldr, image_uuid).unwrap();
    // send the new_event message over the messages socket
    msg_socket
        .send(data, 0)
        .expect("could not send image stored event over zmq socket");
    Ok(())
}

pub fn make_image_deleted_msg<'a>(
    bldr: &'a mut FlatBufferBuilder,
    image_uuid: &'a str,
) -> Result<&'a [u8], std::io::Error> {
    bldr.reset();

    let args = ImageDeletedEventArgs {
        image_uuid: Some(bldr.create_string(image_uuid)),
    };
    let image_deleted_event = ImageDeletedEvent::create(bldr, &args);

    let event_args = EventArgs {
        event_type: EventType::ImageDeletedEvent,
        event: Some(image_deleted_event.as_union_value()),
    };
    let event = Event::create(bldr, &event_args);
    bldr.finish(event, None);

    Ok(bldr.finished_data())
}

pub fn send_image_deleted_event(
    msg_socket: &mut Socket,
    bldr: &mut FlatBufferBuilder,
    image_uuid: &str,
) -> Result<(), std::io::Error> {
    let data = make_image_deleted_msg(bldr, image_uuid).unwrap();
    // send the new_event message over the messages socket
    msg_socket
        .send(data, 0)
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
    fn test_compute_event_type_bytes_filters() -> std::io::Result<()> {
        compute_event_type_bytes_filters().unwrap();

        Ok(())
    }

    #[test]
    fn test_write_new_image_event_to_file() -> std::io::Result<()> {
        let mut bldr = FlatBufferBuilder::new();

        let image_uuid = Uuid::new_v4().to_string();
        let image_format = "png".to_string();
        let image = Vec::<u8>::new();

        let args = NewImageEventArgs {
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
