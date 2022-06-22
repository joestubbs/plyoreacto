"""
A plugin that reads events of type 2 and creates events of type 1.
"""
import datetime
import random
import time
import zmq

def main():
    """ main method of plugin B"""

    context = zmq.Context()

    # ip = '172.17.0.1'
    # ip = '127.0.0.1'
    ip = 'broker'

    # This is the subscriptions socket; it is a SUB type for all plugins.
    new_events = context.socket(zmq.SUB)
    # connect to the new_events socket on port 5560
    new_events.connect(f"tcp://{ip}:5560")
    # This plugin is interested in events of type 1.
    new_events.setsockopt_string(zmq.SUBSCRIBE, "type:2,")

    # This is the socket used by clients for publishing mew events; it is a PUB type for all plugins.
    outgoing = context.socket(zmq.PUB)
    outgoing.connect(f"tcp://{ip}:5559")
    print(f"{datetime.datetime.now()}: plugin B connected to sockets")

    # sleep to make sure broker and plugins are running... need to fix this with proper sync
    time.sleep(10)
    print(f"{datetime.datetime.now()}: plugin B waking up from initial sleep...")
    # Send initial type 1 event to get everything going
    initial_event_msg = "type:1,value:0"
    outgoing.send_string(initial_event_msg)
    print(f"{datetime.datetime.now()}: plugin B sent initial message")

    # Process 5 new events
    total = 0
    while total < 5:
        string = new_events.recv_string()
        event_type, message = string.split(",")
        print(f"got message: {message}")
        # create a new event of type 1 in response to the type 1 event.
        event_msg = f"value:{random.randrange(1,10)}"
        print(f"  >> sending {event_msg}\n")
        type_1_event_msg= f"type:1,{event_msg}"
        # publish the new event
        outgoing.send_string(type_1_event_msg)
        total += 1

    # shutdown
    new_events.close()
    outgoing.close()
    context.term()


if __name__ == "__main__":
    print(f"{datetime.datetime.now()}: plugin B starting up...")
    main()
    print("plugin B exiting.")