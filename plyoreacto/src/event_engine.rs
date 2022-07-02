use std::thread;

use crate::events::get_event_type_bytes_filter;

use super::image_score_plugin;
use super::image_store_plugin;
use super::new_image_plugin;
use flatbuffers::FlatBufferBuilder;
use zmq::Socket;

fn get_outgoing_socket(context: &zmq::Context) -> std::io::Result<Socket> {
    let outgoing = context
        .socket(zmq::PUB)
        .expect("Engine could not create outgoing socket");
    outgoing
        .bind("tcp://*:5560")
        .expect("Engine could not bind outgoing TCP socket");
    outgoing
        .bind("inproc://events")
        .expect("Engine could not bind outgoing inproc socket");
    Ok(outgoing)
}

fn get_incoming_socket(context: &zmq::Context) -> std::io::Result<Socket> {
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
    let filter = String::new();
    incoming
        .set_subscribe(filter.as_bytes())
        .expect("Engine could not subscribe to all events on incoming socket");
    Ok(incoming)
}

fn start_plugin<F>(
    ctx: &zmq::Context,
    plugin_id: i32,
    subscriptions: &[String],
    f: F,
) -> std::io::Result<()>
where
    F: FnOnce(&mut Socket, &mut Socket, &mut FlatBufferBuilder) -> std::io::Result<()>
        + std::marker::Send
        + 'static,
{
    // Create the socket that plugin will use to publish new events
    let mut pub_socket = ctx
        .socket(zmq::PUB)
        .expect("could not create messages socket.");
    pub_socket
        .connect("inproc://messages")
        .expect("could not connect to subscriptions socket");
    println!("plugin {} connected to pub socket.", plugin_id);

    // Create the socket that plugin will use to subscribe to events
    let mut sub_socket = ctx
        .socket(zmq::SUB)
        .expect("could not create subscription socket.");
    sub_socket
        .connect("inproc://events")
        .expect("could not connect to subscriptions socket");
    // Subscribe only to events of interest
    for sub in subscriptions {
        let filter_bytes = get_event_type_bytes_filter(sub).expect("could not get bytes filter");
        sub_socket
            .set_subscribe(&filter_bytes)
            .expect("could not subscribe to event type");
    }

    // Create the sync socket that plugin will use to sync with engine and other plugins
    let sync = ctx
        .socket(zmq::REQ)
        .expect("plugin could not create sync socket.");
    let sync_endpoint_port = 5000 + plugin_id;
    let sync_endpoint = format!("inproc://sync-{}", sync_endpoint_port);
    sync.connect(&sync_endpoint)
        .expect("plugin could not connect to sync socket.");
    println!("plugin {} connected to sync socket.", plugin_id);

    // start the plugin thread
    thread::spawn(move || {
        // connect to and send sync message on sync socket
        let msg = "ready";
        sync.send(msg, 0)
            .expect("plugin could not send sync message");
        println!("plugin {} sent sync message.", plugin_id);
        // wait for reply from engine
        let _msg = sync
            .recv_msg(0)
            .expect("plugin got error trying to receive sync reply");
        println!(
            "plugin {} got sync reply, will now block for messages",
            plugin_id
        );

        let mut bldr = FlatBufferBuilder::new();

        // now execute the actual plugin function
        println!("Executing start function for plugin {}", plugin_id);
        f(&mut pub_socket, &mut sub_socket, &mut bldr)
            .expect("got error executing plugin start function");
    });

    Ok(())
}

fn sync_plugins(context: zmq::Context) -> std::io::Result<()> {
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

    Ok(())
}

fn start_plugins(context: zmq::Context) -> std::io::Result<()> {
    // start each plugin
    start_plugin(&context, 0, &[], new_image_plugin::start).expect("could not start plugin");
    start_plugin(
        &context,
        1,
        &["NewImageEvent".to_string()],
        image_score_plugin::start,
    )
    .expect("could not start plugin");
    start_plugin(
        &context,
        2,
        &["ImageScoredEvent".to_string()],
        image_store_plugin::start,
    )
    .expect("could not start plugin");

    sync_plugins(context).unwrap();
    Ok(())
}

pub fn event_engine() -> std::io::Result<()> {
    println!("Starting EVENT engine");
    // zmq context to be used by this engine and all plugin threads
    let context = zmq::Context::new();

    // incoming and outgoing sockets for the engine
    let outgoing = get_outgoing_socket(&context).expect("could not create outgoing socket");
    let incoming = get_incoming_socket(&context).expect("could not create incoming socket");

    // start plugins in their own thread
    start_plugins(context).unwrap();

    // proxy from incoming to outgoing sockets;
    // this call blocks forever
    let _result = zmq::proxy(&incoming, &outgoing)
        .expect("Engine got error running proxy; socket was closed?");

    // should never get here
    Ok(())
}
