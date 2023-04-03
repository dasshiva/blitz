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

pub fn u64_to_u8(buf: u64) -> [u8; 8] {
  let mut num = [0u8; 8];
  num[0] = (buf << 0) as u8;
  num[1] = (buf << 8) as u8;
  num[2] = (buf << 16) as u8;
  num[3] = (buf << 24) as u8;
  num[4] = (buf << 32) as u8;
  num[5] = (buf << 40) as u8;
  num[6] = (buf << 48) as u8;
  num[7] = (buf << 56) as u8;
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
