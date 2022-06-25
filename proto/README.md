# Prototypes
This directory contains sample/prototype code for trying out an approach based on ZMQ. 

There are currently two examples in this directory:
1) docker-compose-basic.yml -- This is just the basic pub-sub weather station example from the zmq docs.
   It makes use of client.py and server.py

2) docker-compose.yml -- This is the more realistic example.
   It makes use of broker.py, pluginA.py, and pluginB.py.
   "broker.py" represents the engine and also includes a "compiled" plugin (plugin C).
   plugin A and plugin B represnt two different plugin implementations (running in their own containers). 
   plugin A and plugin B use the tcp transport while plugin C uses inproc.

All code runs out of the same image (jstubbs/pyzmqevents) by changing the command.

There is a Makefile to help remember the commands to run.

## Build the Python Image

Even if you just want to run the Rust engine, you still need to build the Python image. Use the 
Makefile to do that:

```
$ make build
```

## Run the Python Example

You can run the Python broker and the plugins using the docker-compose file:

```
$ make up
```

