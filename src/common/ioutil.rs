/******************************************************************************
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0.  If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 * 
 * Software distributed under the License is distributed on an "AS IS" basis, 
 * WITHOUT WARRANTY OF ANY KIND, either express or implied. See the License for 
 * the specific language governing rights and limitations under the License.
 *
 * The Original Code is: RustyMem
 * The Initial Developer of the Original Code is: William Wong (williamw520@gmail.com)
 * Portions created by William Wong are Copyright (C) 2013 William Wong, All Rights Reserved.
 *
 ******************************************************************************/



use std::vec;


/// Platform independent, language independent way of packing data into byte buffer


/// Pack a u8 into byte buffer in big-endian (network order)
pub fn pack_u8_be(buf: &mut [u8], offset: uint, value: u8) -> uint {
    buf[offset] = value;
    offset + 1
}

/// Pack a u16 into byte buffer in big-endian (network order)
pub fn pack_u16_be(buf: &mut [u8], offset: uint, value: u16) -> uint {
    buf[offset + 0] = (value >> 8) as u8;
    buf[offset + 1] = (value >> 0) as u8;
    offset + 2
}

/// Pack a u32 into byte buffer in big-endian (network order)
pub fn pack_u32_be(buf: &mut [u8], offset: uint, value: u32) -> uint {
    buf[offset + 0] = (value >> 24) as u8;
    buf[offset + 1] = (value >> 16) as u8;
    buf[offset + 2] = (value >> 8) as u8;
    buf[offset + 3] = (value >> 0) as u8;
    offset + 4
}

/// Pack a u64 into byte buffer in big-endian (network order)
pub fn pack_u64_be(buf: &mut [u8], offset: uint, value: u64) -> uint {
    buf[offset + 0] = (value >> 56) as u8;
    buf[offset + 1] = (value >> 48) as u8;
    buf[offset + 2] = (value >> 40) as u8;
    buf[offset + 3] = (value >> 32) as u8;
    buf[offset + 4] = (value >> 24) as u8;
    buf[offset + 5] = (value >> 16) as u8;
    buf[offset + 6] = (value >> 8) as u8;
    buf[offset + 7] = (value >> 0) as u8;
    offset + 8
}

/// Unpack a u8 from byte buffer in big-endian (network order)
pub fn unpack_u8_be(buf: &[u8], offset: uint) -> u8 {
    buf[offset]
}

/// Unpack a u16 from byte buffer in big-endian (network order)
pub fn unpack_u16_be(buf: &[u8], offset: uint) -> u16 {
    ( ((buf[offset + 0] as u16) & 0xFF) << 8 ) |
    ( ((buf[offset + 1] as u16) & 0xFF)      )
}

/// Unpack a u32 from byte buffer in big-endian (network order)
pub fn unpack_u32_be(buf: &[u8], offset: uint) -> u32 {
    ( ((buf[offset + 0] as u32) & 0xFF) << 24 ) |
    ( ((buf[offset + 1] as u32) & 0xFF) << 16 ) |
    ( ((buf[offset + 2] as u32) & 0xFF) << 8  ) |
    ( ((buf[offset + 3] as u32) & 0xFF)       )
}

/// Unpack a u64 from byte buffer in big-endian (network order)
pub fn unpack_u64_be(buf: &[u8], offset: uint) -> u64 {
    ( ((buf[offset + 0] as u64) & 0xFF) << 56 ) |
    ( ((buf[offset + 1] as u64) & 0xFF) << 48 ) |
    ( ((buf[offset + 2] as u64) & 0xFF) << 40 ) |
    ( ((buf[offset + 3] as u64) & 0xFF) << 32 ) |
    ( ((buf[offset + 4] as u64) & 0xFF) << 24 ) |
    ( ((buf[offset + 5] as u64) & 0xFF) << 16 ) |
    ( ((buf[offset + 6] as u64) & 0xFF) << 8  ) |
    ( ((buf[offset + 7] as u64) & 0xFF)       )
}

pub fn pack_str(buf: &mut [u8], offset: uint, str_value: &str) -> uint {
    let str_bytes = str_value.as_bytes();
    return copy_bytes(buf, offset, str_bytes, 0, str_bytes.len());
}

pub fn copy_bytes(to_buf: &mut [u8],  to_offset: uint,  from_buf: &[u8],  from_offset: uint,  len: uint) -> uint {
    let to_slice = to_buf.mut_slice(to_offset, to_offset + len);
    let from_slice = from_buf.slice(from_offset, from_offset + len);
    vec::bytes::copy_memory(to_slice, from_slice, len);
    to_offset + len
}

pub fn fold_bytes(bytes: &[u8]) -> u32 {
    let mut value = 0u32;
    for i in range(0, bytes.len() / 4) {
        let val4 = unpack_u32_be(bytes, i * 4);
        value = value ^ val4;
    }
    value
}


#[test]
fn test() {
    //println( fmt!("%?", clean_split("a.b.c", '.')) );
    let mut buf = vec::from_elem(32, 0u8);
    let mut offset;

    offset = pack_u8_be(buf, 0, 1);
    offset = pack_u8_be(buf, offset, 2);
    println( fmt!("%? %?", buf, offset) );

    offset = pack_u16_be(buf, 0, 0x0102);
    println( fmt!("%? %?", buf, offset) );

    offset = pack_str(buf, offset, "ABCD");
    println( fmt!("%? %?", buf, offset) );

    offset = pack_u16_be(buf, 0, 12345);
    println( fmt!("%? %?", buf, offset) );
    println( fmt!("%? %?", unpack_u16_be(buf, 0), offset) );

    offset = pack_u32_be(buf, 0, 12345678);
    println( fmt!("%? %?", buf, offset) );
    println( fmt!("%? %?", unpack_u32_be(buf, 0), offset) );

    offset = pack_u64_be(buf, 0, 12345678901234);
    println( fmt!("%? %?", buf, offset) );
    println( fmt!("%? %?", unpack_u64_be(buf, 0), offset) );

    pack_u64_be(buf, 0, 0);
    println( fmt!("%? %?", fold_bytes(buf), buf) );
    pack_u64_be(buf, 0, 1);
    println( fmt!("%? %?", fold_bytes(buf), buf) );
    pack_u64_be(buf, 0, 0x0000000100000002);
    println( fmt!("%? %?", fold_bytes(buf), buf) );

}

