# Plugin driver to facilitate plyoreacto plugins written in Python.
import datetime
import threading

import flatbuffers
import zmq


# network-addressable host of the engine
ENGINE_HOST = 'engine'

# TODO -- "discover" plugins dynamically from a configuration file, etc.
import observer

python_plugins = [
    {
        # NOTE: currently this plugin id *must* match the corresponding id in the Rust engine.
        "plugin_id": 3,
        "subscriptions": [
            "NewImageEvent", 
            "ImageScoredEvent",
            "ImageStoredEvent", 
            "ImageDeletedEvent"
            ],
        "start_function": observer.start
    },
]

def get_pub_socket(context):
    # This is the socket used by clients for publishing mew events; 
    # it is a PUB type for all plugins.
    pub_socket = context.socket(zmq.PUB)
    pub_socket.connect(f"tcp://{ENGINE_HOST}:5559")
    return pub_socket

def get_sub_socket(context):
    # This is the subscriptions socket; it is a SUB type for all plugins.
    sub_socket = context.socket(zmq.SUB)
    # connect to the new_events socket on port 5560
    sub_socket.connect(f"tcp://{ENGINE_HOST}:5560")
    return sub_socket
    # # This plugin is interested in events of type 1.
    # sub_socket.setsockopt_string(zmq.SUBSCRIBE, "type:1,")

def get_event_type_bytes_filter(sub):
    # return a byte array corresponding to the filter needed to subscribe to a message
    if sub == 'NewImageEvent':
        return bytes([12, 0, 0, 0, 8, 0, 14, 0, 7, 0, 8, 0, 8, 0, 0, 0, 0, 0, 0, 1])
    if sub == 'ImageScoredEvent':
        return bytes([12, 0, 0, 0, 8, 0, 12, 0, 7, 0, 8, 0, 8, 0, 0, 0, 0, 0, 0, 2])
    if sub == "ImageStoredEvent":
        return bytes([12, 0, 0, 0, 8, 0, 14, 0, 7, 0, 8, 0, 8, 0, 0, 0, 0, 0, 0, 3])
    if sub == "ImageDeletedEvent":
        return  bytes([12, 0, 0, 0, 8, 0, 14, 0, 7, 0, 8, 0, 8, 0, 0, 0, 0, 0, 0, 4])

def sync_plugin(plugin_id, context):
    # sync with the engine
    sync = context.socket(zmq.REQ)
    # Plugins connect to sync sockets on different ports; the numbering is 0-based, starting at 5000.
    
    sync_port = 5000 + plugin_id
    sync_socket_connect_str = f"tcp://{ENGINE_HOST}:{sync_port}"
    sync.connect(sync_socket_connect_str)
    print(f"{datetime.datetime.now()}: plugin {plugin_id} connected to sync socket: {sync_socket_connect_str}.")
    sync.send_string("ok")
    print(f"{datetime.datetime.now()}: plugin {plugin_id} sent message on sync socket.")
    # wait for reply
    sync.recv_string()
    print(f"{datetime.datetime.now()}: plugin {plugin_id} got reply on sync socket.")
    return True

def start_plugin(plugin, context):
    # create publish and subscribe sockets for the plugins
    pub_socket = get_pub_socket(context)
    sub_socket = get_sub_socket(context)
    plugin_id = plugin['plugin_id']

    # first, subscribe to the events this plugin is interested in.
    for sub in plugin['subscriptions']:
        filter_bytes = get_event_type_bytes_filter(sub)
        sub_socket.setsockopt(zmq.SUBSCRIBE, filter_bytes)
    
    # flatbuffer builder object for sending messages
    bldr = flatbuffers.Builder(1024)

    # sync with the engine
    sync_plugin(plugin_id, context)

    # start plugin in a separate thread
    p_thread = threading.Thread(target=plugin['start_function'], 
                                args=(pub_socket, sub_socket, bldr),
                                daemon=True)
    p_thread.start()
    print(f"{datetime.datetime.now()}: plugin {plugin_id} thread started by driver")
    return p_thread


def main():
    print(f"{datetime.datetime.now()}: Starting python plugin driver")
    
    context = zmq.Context()
    threads = []

    # start plugins in their own thread
    for p in python_plugins:
        threads.append(start_plugin(p, context))

    # wait for all threads to complete
    for t in threads:
        t.join()
    

if __name__ == "__main__":
    main()
