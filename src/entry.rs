// Copyright 2023 RisingWave Labs
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Entry(pub [u8; 4]);

impl Entry {
    const LEN_MASK: u32 = 0x1FFFFFFF;

    pub const NULL_TAG: u32 = 0;
    pub const STRING_TAG: u32 = 1;
    pub const NUMBER_TAG: u32 = 2;
    pub const FALSE_TAG: u32 = 3;
    pub const TRUE_TAG: u32 = 4;
    pub const ARRAY_TAG: u32 = 5;
    pub const OBJECT_TAG: u32 = 6;

    pub const fn tag(self) -> u32 {
        u32::from_ne_bytes(self.0) >> 29
    }

    pub const fn offset(self) -> usize {
        (u32::from_ne_bytes(self.0) & Self::LEN_MASK) as usize
    }

    pub const fn null() -> Self {
        Self::from_u32(Self::NULL_TAG << 29)
    }

    pub const fn false_() -> Self {
        Self::from_u32(Self::FALSE_TAG << 29)
    }

    pub const fn true_() -> Self {
        Self::from_u32(Self::TRUE_TAG << 29)
    }

    pub const fn bool(b: bool) -> Self {
        if b {
            Self::true_()
        } else {
            Self::false_()
        }
    }

    pub const fn number(offset: usize) -> Self {
        assert!(offset <= Self::LEN_MASK as usize, "offset too large");
        Self::from_u32((Self::NUMBER_TAG << 29) | (offset as u32))
    }

    pub const fn string(offset: usize) -> Self {
        assert!(offset <= Self::LEN_MASK as usize, "offset too large");
        Self::from_u32((Self::STRING_TAG << 29) | (offset as u32))
    }

    pub const fn array(offset: usize) -> Self {
        assert!(offset <= Self::LEN_MASK as usize, "offset too large");
        Self::from_u32((Self::ARRAY_TAG << 29) | (offset as u32))
    }

    pub const fn object(offset: usize) -> Self {
        assert!(offset <= Self::LEN_MASK as usize, "offset too large");
        Self::from_u32((Self::OBJECT_TAG << 29) | (offset as u32))
    }

    pub const fn is_number(self) -> bool {
        self.tag() == Self::NUMBER_TAG
    }

    pub const fn is_string(self) -> bool {
        self.tag() == Self::STRING_TAG
    }

    pub const fn is_array(self) -> bool {
        self.tag() == Self::ARRAY_TAG
    }

    pub const fn is_object(self) -> bool {
        self.tag() == Self::OBJECT_TAG
    }

    pub fn set_offset(&mut self, offset: usize) {
        assert!(offset <= Self::LEN_MASK as usize, "offset too large");
        self.0 = ((self.tag() << 29) | (offset as u32)).to_ne_bytes();
    }

    pub const fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    const fn from_u32(value: u32) -> Self {
        Self(value.to_ne_bytes())
    }
}

/// Convert a 4-byte slice to an `Entry`.
///
/// # Panics
///
/// Panics if the slice is not 4 bytes long.
impl From<&[u8]> for Entry {
    fn from(slice: &[u8]) -> Self {
        Entry(slice.try_into().expect("entry must be 4 bytes"))
    }
}

// last 4 bits is the size
pub const NUMBER_ZERO: u8 = 0x0;
pub const NUMBER_I8: u8 = 0x1;
pub const NUMBER_I16: u8 = 0x2;
pub const NUMBER_I32: u8 = 0x4;
pub const NUMBER_I64: u8 = 0x8;
pub const NUMBER_U64: u8 = 0x18;
pub const NUMBER_F64: u8 = 0x28;

/// Returns the size of the number in bytes.
pub const fn number_size(tag: u8) -> usize {
    (tag & 0xF) as usize
}
