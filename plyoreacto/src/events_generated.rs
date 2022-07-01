// automatically generated by the FlatBuffers compiler, do not modify



use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[allow(unused_imports, dead_code)]
pub mod events {

  use core::mem;
  use core::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};

#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_EVENT_TYPE: u8 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_EVENT_TYPE: u8 = 4;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_EVENT_TYPE: [EventType; 5] = [
  EventType::NONE,
  EventType::NewImageEvent,
  EventType::ImageScoredEvent,
  EventType::ImageStoredEvent,
  EventType::ImageDeletedEvent,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct EventType(pub u8);
#[allow(non_upper_case_globals)]
impl EventType {
  pub const NONE: Self = Self(0);
  pub const NewImageEvent: Self = Self(1);
  pub const ImageScoredEvent: Self = Self(2);
  pub const ImageStoredEvent: Self = Self(3);
  pub const ImageDeletedEvent: Self = Self(4);

  pub const ENUM_MIN: u8 = 0;
  pub const ENUM_MAX: u8 = 4;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::NONE,
    Self::NewImageEvent,
    Self::ImageScoredEvent,
    Self::ImageStoredEvent,
    Self::ImageDeletedEvent,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::NONE => Some("NONE"),
      Self::NewImageEvent => Some("NewImageEvent"),
      Self::ImageScoredEvent => Some("ImageScoredEvent"),
      Self::ImageStoredEvent => Some("ImageStoredEvent"),
      Self::ImageDeletedEvent => Some("ImageDeletedEvent"),
      _ => None,
    }
  }
}
impl core::fmt::Debug for EventType {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for EventType {
  type Inner = Self;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = unsafe {
      flatbuffers::read_scalar_at::<u8>(buf, loc)
    };
    Self(b)
  }
}

impl flatbuffers::Push for EventType {
    type Output = EventType;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        unsafe { flatbuffers::emplace_scalar::<u8>(dst, self.0); }
    }
}

impl flatbuffers::EndianScalar for EventType {
  #[inline]
  fn to_little_endian(self) -> Self {
    let b = u8::to_le(self.0);
    Self(b)
  }
  #[inline]
  #[allow(clippy::wrong_self_convention)]
  fn from_little_endian(self) -> Self {
    let b = u8::from_le(self.0);
    Self(b)
  }
}

impl<'a> flatbuffers::Verifiable for EventType {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    u8::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for EventType {}
pub struct EventTypeUnionTableOffset {}

pub enum NewImageEventOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct NewImageEvent<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for NewImageEvent<'a> {
  type Inner = NewImageEvent<'a>;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table { buf, loc } }
  }
}

impl<'a> NewImageEvent<'a> {
  pub const VT_MESSAGE_TYPE: flatbuffers::VOffsetT = 4;
  pub const VT_IMAGE_UUID: flatbuffers::VOffsetT = 6;
  pub const VT_IMAGE_FORMAT: flatbuffers::VOffsetT = 8;
  pub const VT_IMAGE: flatbuffers::VOffsetT = 10;

  #[inline]
  pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    NewImageEvent { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args NewImageEventArgs<'args>
  ) -> flatbuffers::WIPOffset<NewImageEvent<'bldr>> {
    let mut builder = NewImageEventBuilder::new(_fbb);
    if let Some(x) = args.image { builder.add_image(x); }
    if let Some(x) = args.image_format { builder.add_image_format(x); }
    if let Some(x) = args.image_uuid { builder.add_image_uuid(x); }
    if let Some(x) = args.message_type { builder.add_message_type(x); }
    builder.finish()
  }


  #[inline]
  pub fn message_type(&self) -> &'a str {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(NewImageEvent::VT_MESSAGE_TYPE, Some(&"new_image")).unwrap()
  }
  #[inline]
  pub fn image_uuid(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(NewImageEvent::VT_IMAGE_UUID, None)
  }
  #[inline]
  pub fn image_format(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(NewImageEvent::VT_IMAGE_FORMAT, None)
  }
  #[inline]
  pub fn image(&self) -> Option<&'a [u8]> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(NewImageEvent::VT_IMAGE, None).map(|v| v.safe_slice())
  }
}

impl flatbuffers::Verifiable for NewImageEvent<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("message_type", Self::VT_MESSAGE_TYPE, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("image_uuid", Self::VT_IMAGE_UUID, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("image_format", Self::VT_IMAGE_FORMAT, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u8>>>("image", Self::VT_IMAGE, false)?
     .finish();
    Ok(())
  }
}
pub struct NewImageEventArgs<'a> {
    pub message_type: Option<flatbuffers::WIPOffset<&'a str>>,
    pub image_uuid: Option<flatbuffers::WIPOffset<&'a str>>,
    pub image_format: Option<flatbuffers::WIPOffset<&'a str>>,
    pub image: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
}
impl<'a> Default for NewImageEventArgs<'a> {
  #[inline]
  fn default() -> Self {
    NewImageEventArgs {
      message_type: None,
      image_uuid: None,
      image_format: None,
      image: None,
    }
  }
}

pub struct NewImageEventBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> NewImageEventBuilder<'a, 'b> {
  #[inline]
  pub fn add_message_type(&mut self, message_type: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(NewImageEvent::VT_MESSAGE_TYPE, message_type);
  }
  #[inline]
  pub fn add_image_uuid(&mut self, image_uuid: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(NewImageEvent::VT_IMAGE_UUID, image_uuid);
  }
  #[inline]
  pub fn add_image_format(&mut self, image_format: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(NewImageEvent::VT_IMAGE_FORMAT, image_format);
  }
  #[inline]
  pub fn add_image(&mut self, image: flatbuffers::WIPOffset<flatbuffers::Vector<'b , u8>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(NewImageEvent::VT_IMAGE, image);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> NewImageEventBuilder<'a, 'b> {
    let start = _fbb.start_table();
    NewImageEventBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<NewImageEvent<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for NewImageEvent<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("NewImageEvent");
      ds.field("message_type", &self.message_type());
      ds.field("image_uuid", &self.image_uuid());
      ds.field("image_format", &self.image_format());
      ds.field("image", &self.image());
      ds.finish()
  }
}
pub enum ImageLabelScoreOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct ImageLabelScore<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for ImageLabelScore<'a> {
  type Inner = ImageLabelScore<'a>;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table { buf, loc } }
  }
}

impl<'a> ImageLabelScore<'a> {
  pub const VT_LABEL: flatbuffers::VOffsetT = 4;
  pub const VT_PROBABILITY: flatbuffers::VOffsetT = 6;

  #[inline]
  pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    ImageLabelScore { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args ImageLabelScoreArgs<'args>
  ) -> flatbuffers::WIPOffset<ImageLabelScore<'bldr>> {
    let mut builder = ImageLabelScoreBuilder::new(_fbb);
    builder.add_probability(args.probability);
    if let Some(x) = args.label { builder.add_label(x); }
    builder.finish()
  }


  #[inline]
  pub fn label(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(ImageLabelScore::VT_LABEL, None)
  }
  #[inline]
  pub fn probability(&self) -> f32 {
    self._tab.get::<f32>(ImageLabelScore::VT_PROBABILITY, Some(0.0)).unwrap()
  }
}

impl flatbuffers::Verifiable for ImageLabelScore<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("label", Self::VT_LABEL, false)?
     .visit_field::<f32>("probability", Self::VT_PROBABILITY, false)?
     .finish();
    Ok(())
  }
}
pub struct ImageLabelScoreArgs<'a> {
    pub label: Option<flatbuffers::WIPOffset<&'a str>>,
    pub probability: f32,
}
impl<'a> Default for ImageLabelScoreArgs<'a> {
  #[inline]
  fn default() -> Self {
    ImageLabelScoreArgs {
      label: None,
      probability: 0.0,
    }
  }
}

pub struct ImageLabelScoreBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> ImageLabelScoreBuilder<'a, 'b> {
  #[inline]
  pub fn add_label(&mut self, label: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(ImageLabelScore::VT_LABEL, label);
  }
  #[inline]
  pub fn add_probability(&mut self, probability: f32) {
    self.fbb_.push_slot::<f32>(ImageLabelScore::VT_PROBABILITY, probability, 0.0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> ImageLabelScoreBuilder<'a, 'b> {
    let start = _fbb.start_table();
    ImageLabelScoreBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<ImageLabelScore<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for ImageLabelScore<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("ImageLabelScore");
      ds.field("label", &self.label());
      ds.field("probability", &self.probability());
      ds.finish()
  }
}
pub enum ImageScoredEventOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct ImageScoredEvent<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for ImageScoredEvent<'a> {
  type Inner = ImageScoredEvent<'a>;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table { buf, loc } }
  }
}

impl<'a> ImageScoredEvent<'a> {
  pub const VT_MESSAGE_TYPE: flatbuffers::VOffsetT = 4;
  pub const VT_IMAGE_UUID: flatbuffers::VOffsetT = 6;
  pub const VT_SCORES: flatbuffers::VOffsetT = 8;

  #[inline]
  pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    ImageScoredEvent { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args ImageScoredEventArgs<'args>
  ) -> flatbuffers::WIPOffset<ImageScoredEvent<'bldr>> {
    let mut builder = ImageScoredEventBuilder::new(_fbb);
    if let Some(x) = args.scores { builder.add_scores(x); }
    if let Some(x) = args.image_uuid { builder.add_image_uuid(x); }
    if let Some(x) = args.message_type { builder.add_message_type(x); }
    builder.finish()
  }


  #[inline]
  pub fn message_type(&self) -> &'a str {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(ImageScoredEvent::VT_MESSAGE_TYPE, Some(&"image_scored")).unwrap()
  }
  #[inline]
  pub fn image_uuid(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(ImageScoredEvent::VT_IMAGE_UUID, None)
  }
  #[inline]
  pub fn scores(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<ImageLabelScore<'a>>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<ImageLabelScore>>>>(ImageScoredEvent::VT_SCORES, None)
  }
}

impl flatbuffers::Verifiable for ImageScoredEvent<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("message_type", Self::VT_MESSAGE_TYPE, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("image_uuid", Self::VT_IMAGE_UUID, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<ImageLabelScore>>>>("scores", Self::VT_SCORES, false)?
     .finish();
    Ok(())
  }
}
pub struct ImageScoredEventArgs<'a> {
    pub message_type: Option<flatbuffers::WIPOffset<&'a str>>,
    pub image_uuid: Option<flatbuffers::WIPOffset<&'a str>>,
    pub scores: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<ImageLabelScore<'a>>>>>,
}
impl<'a> Default for ImageScoredEventArgs<'a> {
  #[inline]
  fn default() -> Self {
    ImageScoredEventArgs {
      message_type: None,
      image_uuid: None,
      scores: None,
    }
  }
}

pub struct ImageScoredEventBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> ImageScoredEventBuilder<'a, 'b> {
  #[inline]
  pub fn add_message_type(&mut self, message_type: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(ImageScoredEvent::VT_MESSAGE_TYPE, message_type);
  }
  #[inline]
  pub fn add_image_uuid(&mut self, image_uuid: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(ImageScoredEvent::VT_IMAGE_UUID, image_uuid);
  }
  #[inline]
  pub fn add_scores(&mut self, scores: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<ImageLabelScore<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(ImageScoredEvent::VT_SCORES, scores);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> ImageScoredEventBuilder<'a, 'b> {
    let start = _fbb.start_table();
    ImageScoredEventBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<ImageScoredEvent<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for ImageScoredEvent<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("ImageScoredEvent");
      ds.field("message_type", &self.message_type());
      ds.field("image_uuid", &self.image_uuid());
      ds.field("scores", &self.scores());
      ds.finish()
  }
}
pub enum ImageStoredEventOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct ImageStoredEvent<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for ImageStoredEvent<'a> {
  type Inner = ImageStoredEvent<'a>;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table { buf, loc } }
  }
}

impl<'a> ImageStoredEvent<'a> {
  pub const VT_MESSAGE_TYPE: flatbuffers::VOffsetT = 4;
  pub const VT_IMAGE_UUID: flatbuffers::VOffsetT = 6;

  #[inline]
  pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    ImageStoredEvent { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args ImageStoredEventArgs<'args>
  ) -> flatbuffers::WIPOffset<ImageStoredEvent<'bldr>> {
    let mut builder = ImageStoredEventBuilder::new(_fbb);
    if let Some(x) = args.image_uuid { builder.add_image_uuid(x); }
    if let Some(x) = args.message_type { builder.add_message_type(x); }
    builder.finish()
  }


  #[inline]
  pub fn message_type(&self) -> &'a str {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(ImageStoredEvent::VT_MESSAGE_TYPE, Some(&"image_stored")).unwrap()
  }
  #[inline]
  pub fn image_uuid(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(ImageStoredEvent::VT_IMAGE_UUID, None)
  }
}

impl flatbuffers::Verifiable for ImageStoredEvent<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("message_type", Self::VT_MESSAGE_TYPE, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("image_uuid", Self::VT_IMAGE_UUID, false)?
     .finish();
    Ok(())
  }
}
pub struct ImageStoredEventArgs<'a> {
    pub message_type: Option<flatbuffers::WIPOffset<&'a str>>,
    pub image_uuid: Option<flatbuffers::WIPOffset<&'a str>>,
}
impl<'a> Default for ImageStoredEventArgs<'a> {
  #[inline]
  fn default() -> Self {
    ImageStoredEventArgs {
      message_type: None,
      image_uuid: None,
    }
  }
}

pub struct ImageStoredEventBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> ImageStoredEventBuilder<'a, 'b> {
  #[inline]
  pub fn add_message_type(&mut self, message_type: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(ImageStoredEvent::VT_MESSAGE_TYPE, message_type);
  }
  #[inline]
  pub fn add_image_uuid(&mut self, image_uuid: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(ImageStoredEvent::VT_IMAGE_UUID, image_uuid);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> ImageStoredEventBuilder<'a, 'b> {
    let start = _fbb.start_table();
    ImageStoredEventBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<ImageStoredEvent<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for ImageStoredEvent<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("ImageStoredEvent");
      ds.field("message_type", &self.message_type());
      ds.field("image_uuid", &self.image_uuid());
      ds.finish()
  }
}
pub enum ImageDeletedEventOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct ImageDeletedEvent<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for ImageDeletedEvent<'a> {
  type Inner = ImageDeletedEvent<'a>;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table { buf, loc } }
  }
}

impl<'a> ImageDeletedEvent<'a> {
  pub const VT_MESSAGE_TYPE: flatbuffers::VOffsetT = 4;
  pub const VT_IMAGE_UUID: flatbuffers::VOffsetT = 6;

  #[inline]
  pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    ImageDeletedEvent { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args ImageDeletedEventArgs<'args>
  ) -> flatbuffers::WIPOffset<ImageDeletedEvent<'bldr>> {
    let mut builder = ImageDeletedEventBuilder::new(_fbb);
    if let Some(x) = args.image_uuid { builder.add_image_uuid(x); }
    if let Some(x) = args.message_type { builder.add_message_type(x); }
    builder.finish()
  }


  #[inline]
  pub fn message_type(&self) -> &'a str {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(ImageDeletedEvent::VT_MESSAGE_TYPE, Some(&"image_deleted")).unwrap()
  }
  #[inline]
  pub fn image_uuid(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(ImageDeletedEvent::VT_IMAGE_UUID, None)
  }
}

impl flatbuffers::Verifiable for ImageDeletedEvent<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("message_type", Self::VT_MESSAGE_TYPE, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("image_uuid", Self::VT_IMAGE_UUID, false)?
     .finish();
    Ok(())
  }
}
pub struct ImageDeletedEventArgs<'a> {
    pub message_type: Option<flatbuffers::WIPOffset<&'a str>>,
    pub image_uuid: Option<flatbuffers::WIPOffset<&'a str>>,
}
impl<'a> Default for ImageDeletedEventArgs<'a> {
  #[inline]
  fn default() -> Self {
    ImageDeletedEventArgs {
      message_type: None,
      image_uuid: None,
    }
  }
}

pub struct ImageDeletedEventBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> ImageDeletedEventBuilder<'a, 'b> {
  #[inline]
  pub fn add_message_type(&mut self, message_type: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(ImageDeletedEvent::VT_MESSAGE_TYPE, message_type);
  }
  #[inline]
  pub fn add_image_uuid(&mut self, image_uuid: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(ImageDeletedEvent::VT_IMAGE_UUID, image_uuid);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> ImageDeletedEventBuilder<'a, 'b> {
    let start = _fbb.start_table();
    ImageDeletedEventBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<ImageDeletedEvent<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for ImageDeletedEvent<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("ImageDeletedEvent");
      ds.field("message_type", &self.message_type());
      ds.field("image_uuid", &self.image_uuid());
      ds.finish()
  }
}
pub enum EventOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Event<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Event<'a> {
  type Inner = Event<'a>;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table { buf, loc } }
  }
}

impl<'a> Event<'a> {
  pub const VT_EVENT_TYPE: flatbuffers::VOffsetT = 4;
  pub const VT_EVENT: flatbuffers::VOffsetT = 6;

  #[inline]
  pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Event { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args EventArgs
  ) -> flatbuffers::WIPOffset<Event<'bldr>> {
    let mut builder = EventBuilder::new(_fbb);
    if let Some(x) = args.event { builder.add_event(x); }
    builder.add_event_type(args.event_type);
    builder.finish()
  }


  #[inline]
  pub fn event_type(&self) -> EventType {
    self._tab.get::<EventType>(Event::VT_EVENT_TYPE, Some(EventType::NONE)).unwrap()
  }
  #[inline]
  pub fn event(&self) -> Option<flatbuffers::Table<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Table<'a>>>(Event::VT_EVENT, None)
  }
  #[inline]
  #[allow(non_snake_case)]
  pub fn event_as_new_image_event(&self) -> Option<NewImageEvent<'a>> {
    if self.event_type() == EventType::NewImageEvent {
      self.event().map(NewImageEvent::init_from_table)
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn event_as_image_scored_event(&self) -> Option<ImageScoredEvent<'a>> {
    if self.event_type() == EventType::ImageScoredEvent {
      self.event().map(ImageScoredEvent::init_from_table)
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn event_as_image_stored_event(&self) -> Option<ImageStoredEvent<'a>> {
    if self.event_type() == EventType::ImageStoredEvent {
      self.event().map(ImageStoredEvent::init_from_table)
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn event_as_image_deleted_event(&self) -> Option<ImageDeletedEvent<'a>> {
    if self.event_type() == EventType::ImageDeletedEvent {
      self.event().map(ImageDeletedEvent::init_from_table)
    } else {
      None
    }
  }

}

impl flatbuffers::Verifiable for Event<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_union::<EventType, _>("event_type", Self::VT_EVENT_TYPE, "event", Self::VT_EVENT, false, |key, v, pos| {
        match key {
          EventType::NewImageEvent => v.verify_union_variant::<flatbuffers::ForwardsUOffset<NewImageEvent>>("EventType::NewImageEvent", pos),
          EventType::ImageScoredEvent => v.verify_union_variant::<flatbuffers::ForwardsUOffset<ImageScoredEvent>>("EventType::ImageScoredEvent", pos),
          EventType::ImageStoredEvent => v.verify_union_variant::<flatbuffers::ForwardsUOffset<ImageStoredEvent>>("EventType::ImageStoredEvent", pos),
          EventType::ImageDeletedEvent => v.verify_union_variant::<flatbuffers::ForwardsUOffset<ImageDeletedEvent>>("EventType::ImageDeletedEvent", pos),
          _ => Ok(()),
        }
     })?
     .finish();
    Ok(())
  }
}
pub struct EventArgs {
    pub event_type: EventType,
    pub event: Option<flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>>,
}
impl<'a> Default for EventArgs {
  #[inline]
  fn default() -> Self {
    EventArgs {
      event_type: EventType::NONE,
      event: None,
    }
  }
}

pub struct EventBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> EventBuilder<'a, 'b> {
  #[inline]
  pub fn add_event_type(&mut self, event_type: EventType) {
    self.fbb_.push_slot::<EventType>(Event::VT_EVENT_TYPE, event_type, EventType::NONE);
  }
  #[inline]
  pub fn add_event(&mut self, event: flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Event::VT_EVENT, event);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> EventBuilder<'a, 'b> {
    let start = _fbb.start_table();
    EventBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Event<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for Event<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("Event");
      ds.field("event_type", &self.event_type());
      match self.event_type() {
        EventType::NewImageEvent => {
          if let Some(x) = self.event_as_new_image_event() {
            ds.field("event", &x)
          } else {
            ds.field("event", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        EventType::ImageScoredEvent => {
          if let Some(x) = self.event_as_image_scored_event() {
            ds.field("event", &x)
          } else {
            ds.field("event", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        EventType::ImageStoredEvent => {
          if let Some(x) = self.event_as_image_stored_event() {
            ds.field("event", &x)
          } else {
            ds.field("event", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        EventType::ImageDeletedEvent => {
          if let Some(x) = self.event_as_image_deleted_event() {
            ds.field("event", &x)
          } else {
            ds.field("event", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        _ => {
          let x: Option<()> = None;
          ds.field("event", &x)
        },
      };
      ds.finish()
  }
}
#[inline]
#[deprecated(since="2.0.0", note="Deprecated in favor of `root_as...` methods.")]
pub fn get_root_as_event<'a>(buf: &'a [u8]) -> Event<'a> {
  unsafe { flatbuffers::root_unchecked::<Event<'a>>(buf) }
}

#[inline]
#[deprecated(since="2.0.0", note="Deprecated in favor of `root_as...` methods.")]
pub fn get_size_prefixed_root_as_event<'a>(buf: &'a [u8]) -> Event<'a> {
  unsafe { flatbuffers::size_prefixed_root_unchecked::<Event<'a>>(buf) }
}

#[inline]
/// Verifies that a buffer of bytes contains a `Event`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_event_unchecked`.
pub fn root_as_event(buf: &[u8]) -> Result<Event, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root::<Event>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `Event` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_event_unchecked`.
pub fn size_prefixed_root_as_event(buf: &[u8]) -> Result<Event, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root::<Event>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `Event` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_event_unchecked`.
pub fn root_as_event_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<Event<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root_with_opts::<Event<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `Event` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_event_unchecked`.
pub fn size_prefixed_root_as_event_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<Event<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root_with_opts::<Event<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a Event and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `Event`.
pub unsafe fn root_as_event_unchecked(buf: &[u8]) -> Event {
  flatbuffers::root_unchecked::<Event>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed Event and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `Event`.
pub unsafe fn size_prefixed_root_as_event_unchecked(buf: &[u8]) -> Event {
  flatbuffers::size_prefixed_root_unchecked::<Event>(buf)
}
#[inline]
pub fn finish_event_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<Event<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_event_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<Event<'a>>) {
  fbb.finish_size_prefixed(root, None);
}
}  // pub mod events

