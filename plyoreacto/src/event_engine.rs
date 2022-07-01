use super::image_score_plugin;
use super::image_store_plugin;
use super::new_image_plugin;
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

fn start_plugins(context: &mut zmq::Context) -> std::io::Result<()> {
    // call all plugins
    image_score_plugin::image_scored_plugin(context);
    image_store_plugin::image_stored_plugin(context);
    new_image_plugin::new_image_plugin(context);

    Ok(())
}

fn sync_plugins(context: &mut zmq::Context) -> std::io::Result<()> {
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
        sync.bind(&&inproc_addr)
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

pub fn event_engine() -> std::io::Result<()> {
    println!("Starting EVENT engine");
    // zmq context to be used by this engine and all plugin threads
    let mut context = zmq::Context::new();

    // incoming and outgoing sockets for the engine
    let outgoing = get_outgoing_socket(&mut context).expect("could not create outgoing socket");
    let incoming = get_incoming_socket(&mut context).expect("could not create incoming socket");

    start_plugins(&mut context).expect("Could not start plugins");
    sync_plugins(&mut context).expect("Could not sync plugins");

    // this call blocks forever
    let _result = zmq::proxy(&incoming, &outgoing)
        .expect("Engine got error running proxy; socket was closed?");

    // should never get here
    Ok(())
}
