pub fn make_u64(buf: &[u8]) -> u64 {
  let mut num: u64 = 0;
  num |= (buf[7] as u64) << 56;
  num |= (buf[6] as u64) << 48;
  num |= (buf[5] as u64) << 40;
  num |= (buf[4] as u64) << 32;
  num |= (buf[3] as u64) << 24;
  num |= (buf[2] as u64) << 16;
  num |= (buf[1] as u64) << 8;
  num |= (buf[0] as u64) << 0;
  num
}

pub fn make_u32(buf: &[u8]) -> u32 {
  let mut num: u32 = 0;
  num |= (buf[3] as u32) << 24;
  num |= (buf[2] as u32) << 16;
  num |= (buf[1] as u32) << 8;
  num |= (buf[0] as u32) << 0;
  num
}

pub fn make_u16(buf: &[u8]) -> u16 {
  let mut num: u16 = 0;
  num |= (buf[1] as u16) << 8;
  num |= (buf[0] as u16) << 0;
  num
}
