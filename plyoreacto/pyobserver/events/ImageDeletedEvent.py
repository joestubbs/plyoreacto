# automatically generated by the FlatBuffers compiler, do not modify

# namespace: events

import flatbuffers
from flatbuffers.compat import import_numpy
np = import_numpy()

class ImageDeletedEvent(object):
    __slots__ = ['_tab']

    @classmethod
    def GetRootAs(cls, buf, offset=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, offset)
        x = ImageDeletedEvent()
        x.Init(buf, n + offset)
        return x

    @classmethod
    def GetRootAsImageDeletedEvent(cls, buf, offset=0):
        """This method is deprecated. Please switch to GetRootAs."""
        return cls.GetRootAs(buf, offset)
    # ImageDeletedEvent
    def Init(self, buf, pos):
        self._tab = flatbuffers.table.Table(buf, pos)

    # ImageDeletedEvent
    def ImageUuid(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

def ImageDeletedEventStart(builder): builder.StartObject(1)
def Start(builder):
    return ImageDeletedEventStart(builder)
def ImageDeletedEventAddImageUuid(builder, imageUuid): builder.PrependUOffsetTRelativeSlot(0, flatbuffers.number_types.UOffsetTFlags.py_type(imageUuid), 0)
def AddImageUuid(builder, imageUuid):
    return ImageDeletedEventAddImageUuid(builder, imageUuid)
def ImageDeletedEventEnd(builder): return builder.EndObject()
def End(builder):
    return ImageDeletedEventEnd(builder)