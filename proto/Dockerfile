# Image: jstubbs/pyzmqevents

FROM python:3.10
ADD requirements.txt /requirements.txt
RUN pip install -r /requirements.txt

# basic pub-sub example
ADD client.py /client.py
ADD server.py /server.py

# broker example
ADD broker.py /broker.py
ADD pluginA.py /pluginA.py
ADD pluginB.py /pluginB.py

ENTRYPOINT [ "python", "-u" ]
