"""
Module with utilities for working with Flatbuffer messages representing events.
"""

from events.Event import Event
from events.EventType import EventType
from events import NewImageEvent, ImageScoredEvent, ImageStoredEvent, ImageDeletedEvent


def bytes_to_event(b):
    """
    Takes a bytes array, b, and returns the Flatbuffers event object associated with it.
    """
    try:
        event = Event.GetRootAs(b, 0)
        return event
    except Exception as e:
        print(f"Got exception from GetRootAs: {e}")
    return None


def event_to_typed_event(event):
    """
    Takes a raw Event.Event object and returns a specialized typed event (e.g., NewImageEvent, 
    ImageScoredEvent, etc.) by first checking the type.
    """
    event_type_int = event.EventType()
    if event_type_int == EventType.NewImageEvent:
        union_new_image_event = NewImageEvent.NewImageEvent()
        union_new_image_event.Init(event.Event().Bytes, event.Event().Pos)
        return union_new_image_event
    if event_type_int == EventType.ImageScoredEvent:
        union_image_scored_event = ImageScoredEvent.ImageScoredEvent()
        union_image_scored_event.Init(event.Event().Bytes, event.Event().Pos)
        return union_image_scored_event
    if event_type_int == EventType.ImageStoredEvent:
        union_image_stored_event = ImageStoredEvent.ImageStoredEvent()
        union_image_stored_event.Init(event.Event().Bytes, event.Event().Pos)
        return union_image_stored_event
    if event_type_int == EventType.ImageDeletedEvent:
        union_image_deleted_event = ImageDeletedEvent.ImageDeletedEvent()
        union_image_deleted_event.Init(event.Event().Bytes, event.Event().Pos)
        return union_image_deleted_event


def bytes_to_typed_event(b):
    """
    Convenience wrapper to take a byte array directly to a typed event (NewImageEvent, ImageScoredEvent, etc)
    """
    event = bytes_to_event(b)
    return event_to_typed_event(event)


def get_event_type_str(event):
    """
    Takes a raw Event.Event object and returns the string event type name, in camel case; e.g., "NewImageEvent",
    "ImageScoredEvent", etc.
    """
    # the EventType method returns an integer event
    event_type_int = event.EventType()
    if event_type_int == EventType.NewImageEvent:
        return "NewImageEvent"
    if event_type_int == EventType.ImageScoredEvent:
        return "ImageScoredEvent"
    if event_type_int == EventType.ImageStoredEvent:
        return "ImageStoredEvent"
    if event_type_int == EventType.ImageDeletedEvent:
        return "ImageDeletedEvent"


def get_scored_event_scores(event):
    """
    For event of type ImageScoredEvent, returns a list of Score objects. We include this 
    helper because the generated Python API is kind of annoying.
    """
    scores = []
    if not type(event) == ImageScoredEvent.ImageScoredEvent:
        raise Exception(f"Bad input passed; event must be an ImageScoredEvent; got {type(event)}")
    for i in range(event.ScoresLength()):
        scores.append(event.Scores(i))
    return scores