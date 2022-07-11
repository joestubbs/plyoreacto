# Observer plugin example
# Subscribes to all events and, for now, prints them out to the screen.

import time
import msgevents
from events import NewImageEvent, ImageScoredEvent, ImageStoredEvent, ImageDeletedEvent


def start(pub_socket, sub_socket, bldr):
    messages = 0
    new_image_msgs = 0
    image_scored_msgs = 0
    image_stored_msgs = 0
    image_deleted_msgs = 0
    # counts of exepected stored and deleted, based on the probabilities in the scored events
    expected_stored = 0
    expected_deleted = 0

    # process unlimited messages
    while True:
        msg_bytes = sub_socket.recv(copy=True)
        messages = messages + 1
        print("Observer got a new event\n")
        # convert the byte array to a specialized event
        event = msgevents.bytes_to_typed_event(msg_bytes)
        image_uuid = event.ImageUuid()
        # all events have a uuid:
        print(f"Observer got event for image: {image_uuid}")
        # check the type and convert to the specific message type
        if type(event) == NewImageEvent.NewImageEvent:
            print("Got a NewImageEvent")
            new_image_msgs += 1
        if type(event) == ImageScoredEvent.ImageScoredEvent:
            print("Got an ImageScoredEvent")
            image_scored_msgs += 1
            scores = msgevents.get_scored_event_scores(event)
            for score in scores:
                print(f"score label: {score.Label()}; probability: {score.Probability()}")
                # we're only interested in the lab's score ---
                if score.Label() == b'labrador':
                    if score.Probability() < 0.5:
                        expected_deleted += 1
                    else:
                        expected_stored += 1
                    break
        if type(event) == ImageStoredEvent.ImageStoredEvent:
            print("Got an ImageStoredEvent")
            image_stored_msgs += 1
            # check the probability to determine whether image should be stored or deleted
        if type(event) == ImageDeletedEvent.ImageDeletedEvent:
            print("Got an ImageDeletedEvent")
            image_deleted_msgs += 1
        # we expect each plugin to send 5 events (for a total of 15 events)
        # we'll report out after 15 messages
        if messages == 15:
            print("Final message counts for observer:")
            print(f"New Image Events: {new_image_msgs}")
            print(f"Image Scored Events: {image_scored_msgs}")
            print(f"Image Stored Events: {image_stored_msgs}")
            print(f"Image Deleted Events: {image_deleted_msgs}")
            if image_stored_msgs == expected_stored and image_deleted_msgs == expected_deleted:
                print("Number of stored and deleted messages matched expectations")
            else:
                print(f"ERROR -- Expected {expected_stored} stored messages and {expected_deleted} deleted messages!")
        
