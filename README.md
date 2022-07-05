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

The current Rust implementation of the engine and example Rust plugins live in the `plyoreacto` directory within the project root. This is where new development will take place going forward.
Look in this directory for instructions on building the Rust code and running the example. 

The `proto` directory contains an initial prototype of the engine, written in Python, as well as two prototype Python plugins. The Python engine can pretty much be ignored at this point. The plugins can 
still be used when running the "demo" (as apposed to the "example"), but soon those will also be 
replaced. Look in this directory for instructions on building the Python images. 

The name is a portmanteau of "plyo", coming from the Greek, meaning to increase, and
"reactor", as in the reactor pattern, which was an inspiration for the design, but is also meant to suggest the flexibility (or pliability) provided by supporting multiple programming languages.
