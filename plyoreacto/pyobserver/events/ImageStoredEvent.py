# automatically generated by the FlatBuffers compiler, do not modify

# namespace: events

import flatbuffers
from flatbuffers.compat import import_numpy
np = import_numpy()

class ImageStoredEvent(object):
    __slots__ = ['_tab']

    @classmethod
    def GetRootAs(cls, buf, offset=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, offset)
        x = ImageStoredEvent()
        x.Init(buf, n + offset)
        return x

    @classmethod
    def GetRootAsImageStoredEvent(cls, buf, offset=0):
        """This method is deprecated. Please switch to GetRootAs."""
        return cls.GetRootAs(buf, offset)
    # ImageStoredEvent
    def Init(self, buf, pos):
        self._tab = flatbuffers.table.Table(buf, pos)

    # ImageStoredEvent
    def ImageUuid(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

def ImageStoredEventStart(builder): builder.StartObject(1)
def Start(builder):
    return ImageStoredEventStart(builder)
def ImageStoredEventAddImageUuid(builder, imageUuid): builder.PrependUOffsetTRelativeSlot(0, flatbuffers.number_types.UOffsetTFlags.py_type(imageUuid), 0)
def AddImageUuid(builder, imageUuid):
    return ImageStoredEventAddImageUuid(builder, imageUuid)
def ImageStoredEventEnd(builder): return builder.EndObject()
def End(builder):
    return ImageStoredEventEnd(builder)