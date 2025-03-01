// automatically generated by the FlatBuffers compiler, do not modify



use std::mem;
use std::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::EndianScalar;

#[allow(unused_imports, dead_code)]
pub mod log {

  use std::mem;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::EndianScalar;

// struct Address, aligned to 1
#[repr(C, align(1))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Address {
  x0_: u8,
  x1_: u8,
  x2_: u8,
  x3_: u8,
} // pub struct Address
impl flatbuffers::SafeSliceAccess for Address {}
impl<'a> flatbuffers::Follow<'a> for Address {
  type Inner = &'a Address;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a Address>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a Address {
  type Inner = &'a Address;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<Address>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for Address {
    type Output = Address;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(self as *const Address as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}
impl<'b> flatbuffers::Push for &'b Address {
    type Output = Address;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(*self as *const Address as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}


impl Address {
  pub fn new<'a>(_x0: u8, _x1: u8, _x2: u8, _x3: u8) -> Self {
    Address {
      x0_: _x0.to_little_endian(),
      x1_: _x1.to_little_endian(),
      x2_: _x2.to_little_endian(),
      x3_: _x3.to_little_endian(),

    }
  }
  pub fn x0<'a>(&'a self) -> u8 {
    self.x0_.from_little_endian()
  }
  pub fn x1<'a>(&'a self) -> u8 {
    self.x1_.from_little_endian()
  }
  pub fn x2<'a>(&'a self) -> u8 {
    self.x2_.from_little_endian()
  }
  pub fn x3<'a>(&'a self) -> u8 {
    self.x3_.from_little_endian()
  }
}

pub enum LogOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Log<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Log<'a> {
    type Inner = Log<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Log<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Log {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args LogArgs<'args>) -> flatbuffers::WIPOffset<Log<'bldr>> {
      let mut builder = LogBuilder::new(_fbb);
      builder.add_size_(args.size_);
      if let Some(x) = args.request { builder.add_request(x); }
      if let Some(x) = args.date { builder.add_date(x); }
      if let Some(x) = args.userid { builder.add_userid(x); }
      if let Some(x) = args.identity { builder.add_identity(x); }
      if let Some(x) = args.address { builder.add_address(x); }
      builder.add_code(args.code);
      builder.finish()
    }

    pub const VT_ADDRESS: flatbuffers::VOffsetT = 4;
    pub const VT_IDENTITY: flatbuffers::VOffsetT = 6;
    pub const VT_USERID: flatbuffers::VOffsetT = 8;
    pub const VT_DATE: flatbuffers::VOffsetT = 10;
    pub const VT_REQUEST: flatbuffers::VOffsetT = 12;
    pub const VT_CODE: flatbuffers::VOffsetT = 14;
    pub const VT_SIZE_: flatbuffers::VOffsetT = 16;

  #[inline]
  pub fn address(&self) -> Option<&'a Address> {
    self._tab.get::<Address>(Log::VT_ADDRESS, None)
  }
  #[inline]
  pub fn identity(&self) -> &'a str {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Log::VT_IDENTITY, None).unwrap()
  }
  #[inline]
  pub fn userid(&self) -> &'a str {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Log::VT_USERID, None).unwrap()
  }
  #[inline]
  pub fn date(&self) -> &'a str {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Log::VT_DATE, None).unwrap()
  }
  #[inline]
  pub fn request(&self) -> &'a str {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Log::VT_REQUEST, None).unwrap()
  }
  #[inline]
  pub fn code(&self) -> u16 {
    self._tab.get::<u16>(Log::VT_CODE, Some(0)).unwrap()
  }
  #[inline]
  pub fn size_(&self) -> u64 {
    self._tab.get::<u64>(Log::VT_SIZE_, Some(0)).unwrap()
  }
}

pub struct LogArgs<'a> {
    pub address: Option<&'a  Address>,
    pub identity: Option<flatbuffers::WIPOffset<&'a  str>>,
    pub userid: Option<flatbuffers::WIPOffset<&'a  str>>,
    pub date: Option<flatbuffers::WIPOffset<&'a  str>>,
    pub request: Option<flatbuffers::WIPOffset<&'a  str>>,
    pub code: u16,
    pub size_: u64,
}
impl<'a> Default for LogArgs<'a> {
    #[inline]
    fn default() -> Self {
        LogArgs {
            address: None,
            identity: None, // required field
            userid: None, // required field
            date: None, // required field
            request: None, // required field
            code: 0,
            size_: 0,
        }
    }
}
pub struct LogBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> LogBuilder<'a, 'b> {
  #[inline]
  pub fn add_address(&mut self, address: &'b  Address) {
    self.fbb_.push_slot_always::<&Address>(Log::VT_ADDRESS, address);
  }
  #[inline]
  pub fn add_identity(&mut self, identity: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Log::VT_IDENTITY, identity);
  }
  #[inline]
  pub fn add_userid(&mut self, userid: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Log::VT_USERID, userid);
  }
  #[inline]
  pub fn add_date(&mut self, date: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Log::VT_DATE, date);
  }
  #[inline]
  pub fn add_request(&mut self, request: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Log::VT_REQUEST, request);
  }
  #[inline]
  pub fn add_code(&mut self, code: u16) {
    self.fbb_.push_slot::<u16>(Log::VT_CODE, code, 0);
  }
  #[inline]
  pub fn add_size_(&mut self, size_: u64) {
    self.fbb_.push_slot::<u64>(Log::VT_SIZE_, size_, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> LogBuilder<'a, 'b> {
    let start = _fbb.start_table();
    LogBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Log<'a>> {
    let o = self.fbb_.end_table(self.start_);
    self.fbb_.required(o, Log::VT_IDENTITY,"identity");
    self.fbb_.required(o, Log::VT_USERID,"userid");
    self.fbb_.required(o, Log::VT_DATE,"date");
    self.fbb_.required(o, Log::VT_REQUEST,"request");
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum LogsOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Logs<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Logs<'a> {
    type Inner = Logs<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Logs<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Logs {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args LogsArgs<'args>) -> flatbuffers::WIPOffset<Logs<'bldr>> {
      let mut builder = LogsBuilder::new(_fbb);
      if let Some(x) = args.logs { builder.add_logs(x); }
      builder.finish()
    }

    pub const VT_LOGS: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn logs(&self) -> flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Log<'a>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<flatbuffers::ForwardsUOffset<Log<'a>>>>>(Logs::VT_LOGS, None).unwrap()
  }
}

pub struct LogsArgs<'a> {
    pub logs: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , flatbuffers::ForwardsUOffset<Log<'a >>>>>,
}
impl<'a> Default for LogsArgs<'a> {
    #[inline]
    fn default() -> Self {
        LogsArgs {
            logs: None, // required field
        }
    }
}
pub struct LogsBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> LogsBuilder<'a, 'b> {
  #[inline]
  pub fn add_logs(&mut self, logs: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<Log<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Logs::VT_LOGS, logs);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> LogsBuilder<'a, 'b> {
    let start = _fbb.start_table();
    LogsBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Logs<'a>> {
    let o = self.fbb_.end_table(self.start_);
    self.fbb_.required(o, Logs::VT_LOGS,"logs");
    flatbuffers::WIPOffset::new(o.value())
  }
}

}  // pub mod log

