# plyoreacto

## Introduction
`plyroreacto` provides a simple message proxy for building event-driven applications that utilize a plugins architecture. In `plyoreacto`, events correspond to typed messages, and plugins subscribe to message types they are interested in. 

The main engine is written in Rust and manages events using sockets based on the `zmq` crate. The engine proxies messages between two sockets: `incoming` and `outgoing`. Plugins then connect to the
sockets to subscribe to (`outgoing`) and publish (`incoming`) events. Plugins can utilize either 
`inproc` or `tcp` transports, allowing them to be written in Rust and compiled with the main 
engine for maximum performance or written in different languages and run as standalone processes.


## Message Serialization

The `plyoreacto` architecture makes no assumptions about the serialization format used for messages --
ZeroMQ sockets work with any byte stream. With that said, the current example utilizes Google 
Flatbuffers. For this reason, `flatbuffers` appears as a Cargo dependency. 


## Repository Structure and Name

The current Rust implementation of the engine and example Rust plugins live in the `plyoreacto` directory within the project root. This is where new development will take place going forward. Inside this directory is also the
`pyobserver` directory which contains an example Python plugin. 
Look in this directory for instructions on building the Rust code, Docker images, and running the "example". 

The `proto` directory contains an initial prototype of the engine, written in Python, as well as two prototype Python plugins. The Python engine can be ignored at this point, unless you just prefer to read Python. The 
plugins were used when running the initial "demo" (as apposed to the "example"), which used basic strings 
for messages, but these have been replaces also. As mentioned above, the example now includes a Python 
plugin (`pyobserver`) that uses the Flatbuffers messages. We keep the `proto` directory here for 
posterity. 

The name is a portmanteau of "plyo", the Greek term meaning "to increase", and
"reactor", as in the [reactor pattern](https://en.wikipedia.org/wiki/Reactor_pattern), which was an inspiration for the design, but is also meant to suggest the flexibility ("pliability") provided by supporting multiple programming languages.
