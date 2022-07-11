# Image: jstubbs/pyobserver

FROM python:3.10
ADD requirements.txt /requirements.txt
RUN pip install -r /requirements.txt

ADD events /events
ADD plugins.py /plugins.py
ADD msgevents.py /msgevents.py

ADD observer.py /observer.py

ENTRYPOINT [ "python" ]
# use unbuffered output so logs show up instantly
CMD [ "-u", "/plugins.py" ]