"""
A plugin that reads events of type 1 and creates events of type 2.
"""
import datetime
import random
import zmq

def main():
    """ main method of plugin A"""

    context = zmq.Context()
    # ip = '172.17.0.1'
    # ip = '127.0.0.1'
    ip = 'broker'

    # This is the subscriptions socket; it is a SUB type for all plugins.
    new_events = context.socket(zmq.SUB)
    # connect to the new_events socket on port 5560
    new_events.connect(f"tcp://{ip}:5560")
    # This plugin is interested in events of type 1.
    new_events.setsockopt_string(zmq.SUBSCRIBE, "type:1,")

    # This is the socket used by clients for publishing mew events; it is a PUB type for all plugins.
    outgoing = context.socket(zmq.PUB)
    # outgoing.bind(f"tcp://127.0.0.1:5559")
    # outgoing.bind(f"tcp://172.17.0.1:5559")
    outgoing.connect(f"tcp://{ip}:5559")
    print(f"{datetime.datetime.now()}: plugin A connected to sockets")

    # Process 5 new events
    total = 0
    while total < 5:
        string = new_events.recv_string()
        event_type, message = string.split(",")
        print(f"got message: {message}")
        # create a new event of type 2 in response to the type 1 event.
        event_msg = f"value:{random.randrange(1,10)}"
        print(f"  >> sending {event_msg}\n")
        type_2_event_msg= f"type:2,{event_msg}"
        # publish the new event
        outgoing.send_string(type_2_event_msg)
        total += 1

    # shutdown
    new_events.close()
    outgoing.close()
    context.term()


if __name__ == "__main__":
    print(f"{datetime.datetime.now()}: plugin A starting up...")
    main()
    print("plugin A exiting.")