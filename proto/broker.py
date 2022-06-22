import datetime
import threading
import zmq

# zmq context object; shared between all threads that use inproc in the main engine.
context = zmq.Context()


def plugin_c_thread():
    """
    Function representing a third plugin, plugin C. This function gets called in a 
    separate thread and utilizes the inproc transport for receiving events.
    """
    # This is the subscriptions socket; it is a SUB type for all plugins.
    new_events = context.socket(zmq.SUB)
    # for inproc, we connect to the "events" socket
    new_events.connect("inproc://events")
    # plugin C is interested in events of type 1:
    new_events.setsockopt_string(zmq.SUBSCRIBE, "type:1,")
    print(f"{datetime.datetime.now()}: plugin C connected to events sockets")

    # Process 5 new events
    total = 0
    while total < 5:
        string = new_events.recv_string()
        event_type, message = string.split(",")
        print(f"plugin C got message: {message}")



def main():
    """ main method of broker"""

    # This is the socket used by clients for subscribing to events. 
    # It is a PUB type for the broker since the broker publishes these events.
    outgoing  = context.socket(zmq.PUB)
    outgoing.bind("tcp://*:5560")
    outgoing.bind("inproc://events")
    

    # This is the socket used by clients to publish new events.
    # It is a SUB type for the broker since the broker receives all of these events.
    incoming = context.socket(zmq.SUB)
    incoming.bind("tcp://*:5559")
    incoming.bind("inproc://messages")
    # subscribe to all messages
    incoming.setsockopt_string(zmq.SUBSCRIBE, "")

    print(f"{datetime.datetime.now()}: broker has established incoming and outgoing sockets.")

    # start the plugin C thread
    pc_thread = threading.Thread(target=plugin_c_thread, daemon=True)
    pc_thread.start()

    print(f"{datetime.datetime.now()}: broker now starting proxy")
    zmq.proxy(incoming, outgoing)

    print(f"{datetime.datetime.now()}: broker returning from proxy")
    
    # We never get here...
    pc_thread.join()
    incoming.close()
    outgoing.close()
    context.term()


if __name__ == "__main__":
    print(f"{datetime.datetime.now()}: broker starting up...")
    main()
    print("broker exiting.")