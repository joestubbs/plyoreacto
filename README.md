# plyoreacto

`plyroreacto` provides a simple message proxy for building event-driven applications that utilize a plugins architecture. In
`plyoreacto`, events correspond to typed messages, and plugins subscribe to message types they are interested in. 

The main engine is written in Rust and manages events using sockets based on the `zmq` crate. The engine proxies messages between
two sockets: `incoming` and `outgoing`. Plugins then connect to the sockets to subscribe to (`outgoing`) and publish (`incoming`)
events. Plugins can utilize either `inproc` or `tcp` transports, allowing them to be written in Rust and compiled with the main 
engine for maximum performance or written in different languages and run as standalone processes.


