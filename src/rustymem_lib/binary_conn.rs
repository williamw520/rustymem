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



use std::result::Result;
use std::str;
use std::vec;
use std::rt::io::net::tcp::TcpStream;
use std::rt::io::{Reader, Writer};
use std::unstable::intrinsics;


use common::netutil;
use common::ioutil;


use super::super::MemStatus;
use super::super::MemResult;
use super::super::MemData;
use super::super::MemcachedStat;
use super::proto::ProtoConnection;



//
// Binary Protocol
//

static BP_HEADER_SIZE: uint         = 24u;
static BP_REQUEST_VERSION: u8       = 0x80u8;
static BP_RESPONSE_VERSION: u8      = 0x81u8;

// Binary protocol command opcodes
static BP_OP_Get: u8                    = 0x00;
static BP_OP_Set: u8                    = 0x01;
static BP_OP_Add: u8                    = 0x02;
static BP_OP_Replace: u8                = 0x03;
static BP_OP_Delete: u8                 = 0x04;
static BP_OP_Increment: u8              = 0x05;
static BP_OP_Decrement: u8              = 0x06;
static BP_OP_Quit: u8                   = 0x07;
static BP_OP_Flush: u8                  = 0x08;
static BP_OP_GetQ: u8                   = 0x09;
static BP_OP_Noop: u8                   = 0x0a;
static BP_OP_Version: u8                = 0x0b;
static BP_OP_GetK: u8                   = 0x0c;
static BP_OP_GetKQ: u8                  = 0x0d;
static BP_OP_Append: u8                 = 0x0e;
static BP_OP_Prepend: u8                = 0x0f;
static BP_OP_Stat: u8                   = 0x10;
static BP_OP_SetQ: u8                   = 0x11;
static BP_OP_AddQ: u8                   = 0x12;
static BP_OP_ReplaceQ: u8               = 0x13;
static BP_OP_DeleteQ: u8                = 0x14;
static BP_OP_IncrementQ: u8             = 0x15;
static BP_OP_DecrementQ: u8             = 0x16;
static BP_OP_QuitQ: u8                  = 0x17;
static BP_OP_FlushQ: u8                 = 0x18;
static BP_OP_AppendQ: u8                = 0x19;
static BP_OP_PrependQ: u8               = 0x1a;
static BP_OP_Verbosity: u8              = 0x1b;
static BP_OP_Touch: u8                  = 0x1c;
static BP_OP_GAT: u8                    = 0x1d;
static BP_OP_GATQ: u8                   = 0x1e;
static BP_OP_SASL_list_mechs: u8        = 0x20;
static BP_OP_SASL_Auth: u8              = 0x21;
static BP_OP_SASL_Step: u8              = 0x22;
static BP_OP_RGet: u8                   = 0x30;
static BP_OP_RSet: u8                   = 0x31;
static BP_OP_RSetQ: u8                  = 0x32;
static BP_OP_RAppend: u8                = 0x33;
static BP_OP_RAppendQ: u8               = 0x34;
static BP_OP_RPrepend: u8               = 0x35;
static BP_OP_RPrependQ: u8              = 0x36;
static BP_OP_RDelete: u8                = 0x37;
static BP_OP_RDeleteQ: u8               = 0x38;
static BP_OP_RIncr: u8                  = 0x39;
static BP_OP_RIncrQ: u8                 = 0x3a;
static BP_OP_RDecr: u8                  = 0x3b;
static BP_OP_RDecrQ: u8                 = 0x3c;
static BP_OP_Set_VBucket: u8            = 0x3d;
static BP_OP_Get_VBucket: u8            = 0x3e;
static BP_OP_Del_VBucket: u8            = 0x3f;
static BP_OP_TAP_Connect: u8            = 0x40;
static BP_OP_TAP_Mutation: u8           = 0x41;
static BP_OP_TAP_Delete: u8             = 0x42;
static BP_OP_TAP_Flush: u8              = 0x43;
static BP_OP_TAP_Opaque: u8             = 0x44;
static BP_OP_TAP_VBucket_Set: u8        = 0x45;
static BP_OP_TAP_Checkpoint_Start: u8   = 0x46;
static BP_OP_TAP_Checkpoint_End: u8     = 0x47;



/// Struct for one memcached server
pub struct BinaryConnection {
    server_addr:    ~netutil::HostAddr,
    stream:         Option<TcpStream>,
}



// ProtoConnection implementation for one memcached server
impl ProtoConnection for BinaryConnection {

    //// Storage commands

    fn p_set(&mut self,  key: &str,  data: &[u8],  cas: u64,  flags: u32,  exptime: uint,  noreply: bool) -> MemResult<u64> {
        return self.bc_store_cmd(BP_OP_Set, key, data, cas, flags, exptime, noreply);
    }

    fn p_cas(&mut self,  key: &str,  data: &[u8],  cas: u64,  flags: u32,  exptime: uint,  noreply: bool) -> MemResult<u64> {
        // set and cas are the same in binary protocol.
        return self.bc_store_cmd(BP_OP_Set, key, data, cas, flags, exptime, noreply);
    }

    fn p_add(&mut self,  key: &str,  data: &[u8],  cas: u64,  flags: u32,  exptime: uint,  noreply: bool) -> MemResult<u64> {
        return self.bc_store_cmd(BP_OP_Add, key, data, cas, flags, exptime, noreply);
    }

    fn p_replace(&mut self,  key: &str,  data: &[u8],  cas: u64,  flags: u32,  exptime: uint,  noreply: bool) -> MemResult<u64> {
        return self.bc_store_cmd(BP_OP_Replace, key, data, cas, flags, exptime, noreply);
    }


    fn p_append(&mut self,  key: &str,  data: &[u8],  noreply: bool) -> MemResult<u64> {
        return self.bc_append_cmd(BP_OP_Append, key, data, noreply);
    }

    fn p_prepend(&mut self,  key: &str,  data: &[u8],  noreply: bool) -> MemResult<u64> {
        return self.bc_append_cmd(BP_OP_Prepend, key, data, noreply);
    }


    //// Data command
    
    fn p_touch(&mut self, key: &str, exptime: uint, _ /*noreply*/: bool) -> MemStatus {
        let key_bytes = key.as_bytes();
        let mut header = BinaryConnection::new_req_header(BP_OP_Touch, key_bytes.len() as u16, 4u8, 0, 0);
        debug!( fmt!("  req: %?", header) );

        let mut body = vec::from_elem(header.body_len as uint, 0u8);
        let mut offset = 0;
        offset = ioutil::pack_u32_be(body, offset, exptime as u32);
        ioutil::copy_bytes(body, offset, key_bytes, 0, key_bytes.len());

        self.write_header(&header);
        self.write_data(body);

        self.read_header(&mut header);
        debug!( fmt!("  res: %?", header) );
        let buf = self.read_upto(header.get_data_len());
        debug!( fmt!("  data: %?", str::from_utf8(buf)) );

        return MemStatus::map_status(header.status_vbucket);
    }

    fn p_incr(&mut self, key: &str, inc_amount: u64, init_value: u64, exptime: uint, noreply: bool) -> MemResult<u64> {
        return self.bc_inc_cmd(BP_OP_Increment, key, exptime, inc_amount, init_value, noreply);
    }

    fn p_decr(&mut self, key: &str, dec_amount: u64, init_value: u64, exptime: uint, noreply: bool) -> MemResult<u64> {
        return self.bc_inc_cmd(BP_OP_Decrement, key, exptime, dec_amount, init_value, noreply);
    }


    fn p_delete(&mut self, key: &str, _ /*noreply*/: bool) -> MemStatus {
        let key_bytes = key.as_bytes();
        let mut header = BinaryConnection::new_req_header(BP_OP_Delete, key_bytes.len() as u16, 0, 0, 0);
        debug!( fmt!("  req: %?", header) );

        let mut body = vec::from_elem(header.body_len as uint, 0u8);
        ioutil::copy_bytes(body, 0, key_bytes, 0, key_bytes.len());

        self.write_header(&header);
        self.write_data(body);

        self.read_header(&mut header);
        debug!( fmt!("  res: %?", header) );
        let buf = self.read_upto(header.get_data_len());
        debug!( fmt!("  data: %?", str::from_utf8(buf)) );

        return MemStatus::map_status(header.status_vbucket);
    }


    //// Retrieval command

    fn p_get(&mut self, keys: &[&str]) -> ~[MemData] {
        return self.p_gets(keys);
    }

    fn p_gets(&mut self, keys: &[&str]) -> ~[MemData] {

        // Return dummy data to cut out network access, for benchmarking.
        // if true {
        //     return ~[];
        // }

        for i in range(0, keys.len() - 1) {
            let key_bytes = keys[i].as_bytes();
            let header = BinaryConnection::new_req_header(BP_OP_GetKQ, key_bytes.len() as u16, 0, 0, 0);
            debug!( fmt!("  req: %?", header) );

            let mut body = vec::from_elem(header.body_len as uint, 0u8);
            ioutil::copy_bytes(body, 0, key_bytes, 0, key_bytes.len());

            self.write_header(&header);
            self.write_data(body);
        }

        let i = keys.len() - 1;
        let key_bytes = keys[i].as_bytes();
        let mut header = BinaryConnection::new_req_header(BP_OP_GetK, key_bytes.len() as u16, 0, 0, 0);
        debug!( fmt!("  req: %?", header) );

        let mut body = vec::from_elem(header.body_len as uint, 0u8);
        ioutil::copy_bytes(body, 0, key_bytes, 0, key_bytes.len());

        self.write_header(&header);
        self.write_data(body);

        let mut mdata_list = ~[];
        loop {
            self.read_header(&mut header);
            debug!( fmt!("  res: %?", header) );
            let extra   = self.read_upto(header.extra_len as uint);
            let key     = self.read_upto(header.key_len as uint);
            let data    = self.read_upto(header.get_data_len());
            if header.status_vbucket == 0 {
                let mdata = MemData {
                    key:        str::from_utf8(key),
                    flags:      ioutil::unpack_u32_be(extra, 0),
                    cas:        header.cas,
                    data:       data
                };
                mdata_list.push(mdata);
            }
            // Look for the last sent request's opcode BP_OP_GetK terminating the BP_OP_GetKQ sequence.
            if header.opcode == BP_OP_GetK {
                break;
            }
        }

        mdata_list
    }


    //// Other commands

    fn p_version(&mut self) -> Result<~str, ~str> {
        let mut header: PacketHeader = BinaryConnection::new_req_header(BP_OP_Version, 0, 0, 0, 0);
        debug!( fmt!("  req: %?", header) );
        self.write_header(&header);
        self.read_header(&mut header);
        debug!( fmt!("  res: %?", header) );
        let buf = self.read_upto(header.get_data_len());
        Ok(str::from_utf8(buf))
    }

    fn p_verbosity(&mut self, verbosity: u32, _ /*noreply*/: bool) -> MemStatus {
        let mut header = BinaryConnection::new_req_header(BP_OP_Verbosity, 0, 4u8, 0, 0);
        debug!( fmt!("  req: %?", header) );

        let mut body = [0u8, ..4];
        ioutil::pack_u32_be(body, 0, verbosity);

        self.write_header(&header);
        self.write_data(body);

        self.read_header(&mut header);
        debug!( fmt!("  res: %?", header) );
        let buf = self.read_upto(header.get_data_len());
        debug!( fmt!("  data: %?", str::from_utf8(buf)) );

        return MemStatus::map_status(header.status_vbucket);
    }

    fn p_flush(&mut self, delay_in_seconds: uint, _ /*noreply*/: bool) -> MemStatus {
        let mut header = BinaryConnection::new_req_header(BP_OP_Flush, 0, 4u8, 0, 0);
        debug!( fmt!("  req: %?", header) );

        let mut body = [0u8, ..4];
        ioutil::pack_u32_be(body, 0, delay_in_seconds as u32);

        self.write_header(&header);
        self.write_data(body);

        self.read_header(&mut header);
        debug!( fmt!("  res: %?", header) );

        return MemStatus::map_status(header.status_vbucket);
    }

    fn p_stats(&mut self) -> ~[MemcachedStat] {
        let mut header = BinaryConnection::new_req_header(BP_OP_Stat, 0, 0, 0, 0);
        debug!( fmt!("  req: %?", header) );

        self.write_header(&header);

        let mut stats : ~[MemcachedStat] = ~[];
        loop {
            self.read_header(&mut header);
            //debug!( fmt!("  res: %?", header) );
            if header.key_len == 0 && header.get_data_len() == 0 {
                break;
            }
            let name  = self.read_upto(header.key_len as uint);
            let value = self.read_upto(header.get_data_len());
            //debug!( fmt!("  stat: %? = %?", str::from_utf8(name), str::from_utf8(value)) );
            stats.push(MemcachedStat {
                    name:   str::from_utf8(name),
                    value:  str::from_utf8(value)
                });
        }

        return stats;
    }

    fn p_quit(&mut self) -> MemStatus {
        let mut header = BinaryConnection::new_req_header(BP_OP_Quit, 0, 0, 0, 0);
        debug!( fmt!("  req: %?", header) );
        self.write_header(&header);
        self.read_header(&mut header);
        debug!( fmt!("  res: %?", header) );
        return MemStatus::map_status(header.status_vbucket);
    }

    // Server config
    fn p_get_server_addr(&self) -> ~str {
        return self.server_addr.to_str();
    }


}


impl BinaryConnection {

    pub fn new_connection(server_addr: ~netutil::HostAddr) -> ~BinaryConnection {
        debug!("new_connection() enter");

        let stream = TcpStream::connect(server_addr.get_sock_addr());
        if stream.is_none() {
            fail!("connect() failed")
        }

        return ~BinaryConnection {
            server_addr:    server_addr,
            stream:         stream,
        };
    }


    fn bc_store_cmd(&mut self,  opcode: u8,  key: &str,  data: &[u8], cas: u64,  flags: u32,  exptime: uint,  _ /*noreply*/: bool) -> MemResult<u64> {
        let key_bytes = key.as_bytes();
        let mut header = BinaryConnection::new_req_header(opcode, key_bytes.len() as u16, 4u8 + 4, data.len(), cas);
        debug!( fmt!("  req: %?", header) );

        let mut body = vec::from_elem(header.body_len as uint, 0u8);
        let mut offset = 0;
        offset = ioutil::pack_u32_be(body, offset, flags);
        offset = ioutil::pack_u32_be(body, offset, exptime as u32);
        offset = ioutil::copy_bytes(body, offset, key_bytes, 0, key_bytes.len());
        ioutil::copy_bytes(body, offset, data, 0, data.len());

        self.write_header(&header);
        self.write_data(body);

        self.read_header(&mut header);
        debug!( fmt!("  res: %?", header) );
        let buf = self.read_upto(header.get_data_len());
        debug!( fmt!("  data: %?", str::from_utf8(buf)) );

        return MemResult::<u64> {
            status:     MemStatus::map_status(header.status_vbucket),
            value:      header.cas
        };
    }

    fn bc_append_cmd(&mut self,  opcode: u8,  key: &str,  data: &[u8], _ /*noreply*/: bool) -> MemResult<u64> {
        let key_bytes = key.as_bytes();
        let mut header = BinaryConnection::new_req_header(opcode, key_bytes.len() as u16, 0, data.len(), 0);
        debug!( fmt!("  req: %?", header) );

        let mut body = vec::from_elem(header.body_len as uint, 0u8);
        let mut offset = 0;
        offset = ioutil::copy_bytes(body, offset, key_bytes, 0, key_bytes.len());
        ioutil::copy_bytes(body, offset, data, 0, data.len());

        self.write_header(&header);
        self.write_data(body);

        self.read_header(&mut header);
        debug!( fmt!("  res: %?", header) );
        let buf = self.read_upto(header.get_data_len());
        debug!( fmt!("  data: %?", str::from_utf8(buf)) );

        return MemResult::<u64> {
            status:     MemStatus::map_status(header.status_vbucket),
            value:      header.cas
        };
    }

    fn bc_inc_cmd(&mut self,  opcode: u8,  key: &str,  exptime: uint,  inc_amount: u64, init_value: u64,  _ /*noreply*/: bool) -> MemResult<u64> {
        let key_bytes = key.as_bytes();
        let mut header = BinaryConnection::new_req_header(opcode, key_bytes.len() as u16, 8u8 + 8 + 4, 0, 0);
        debug!( fmt!("  req: %?", header) );

        let mut body = vec::from_elem(header.body_len as uint, 0u8);
        let mut offset = 0;
        offset = ioutil::pack_u64_be(body, offset, inc_amount);
        offset = ioutil::pack_u64_be(body, offset, init_value);
        offset = ioutil::pack_u32_be(body, offset, exptime as u32);
        ioutil::copy_bytes(body, offset, key_bytes, 0, key_bytes.len());

        self.write_header(&header);
        self.write_data(body);

        self.read_header(&mut header);
        debug!( fmt!("  res: %?", header) );
        let buf = self.read_upto(header.get_data_len());
        debug!( fmt!("  data: %?", str::from_utf8(buf)) );
        let new_value = if buf.len() == 8 && header.status_vbucket == 0 { ioutil::unpack_u64_be(buf, 0) } else { 0 };

        return MemResult::<u64> {
            status:     MemStatus::map_status(header.status_vbucket),
            value:      new_value
        };
    }


    // Create a header struct, on stack.
    fn new_req_header(opcode: u8, key_len: u16, extra_len: u8, data_len: uint, cas: u64) -> PacketHeader {
        return PacketHeader {
            magic:      BP_REQUEST_VERSION,
            opcode:     opcode,
            key_len:    key_len,
            extra_len:  extra_len,
            data_type:  0u8,
            status_vbucket: 0u16,
            body_len:   (key_len as uint + extra_len as uint + data_len) as u32,
            client_ctx: 0u32,
            cas:        cas
        };
    }

    fn write_data(&mut self, data: &[u8]) {
        //debug!( fmt!("write data: %?", data) );
        self.stream.write(data);
    }

    fn write_header(&mut self, header: &PacketHeader) {
        let mut buf = [0u8, ..BP_HEADER_SIZE];
        header.pack(buf, 0);
        //debug!( fmt!("req buf: %?", buf) );
        self.write_data(buf);
    }

    fn read_header(&mut self, header: &mut PacketHeader) {
        let buf = self.read_upto(BP_HEADER_SIZE);
        header.unpack(buf, 0);
    }

    fn read_any(&mut self, buf: &mut [u8]) {
        self.stream.read(buf);
    }

    fn read_upto(&mut self, len_to_read: uint) -> ~[u8] {
        let mut buf = vec::from_elem(len_to_read, 0u8);
        self.read_buf_upto(buf, 0, len_to_read);
        return buf;
    }

    fn read_buf_upto(&mut self, buf: &mut [u8], offset: uint, len_to_read: uint) {
        let mut total_read = 0u;
        while total_read < len_to_read {
            let remaining_len = len_to_read - total_read;
            let slice_buf = buf.mut_slice(offset, offset + remaining_len);
            match self.stream.read(slice_buf) {
                Some(read_len) => total_read = total_read + read_len,
                None => break //println( fmt!("read_upto error: %?", s) ); break; }
            }
        }
    }


}


// [#packed]
// packed not working in Rust yet.
// check out std::unstable::intrinsics::to_be16, vec::raw::buf_as_slice for working with raw memory
struct PacketHeader {
    magic:      u8,
    opcode:     u8,
    key_len:    u16,
    extra_len:  u8,
    data_type:  u8,
    status_vbucket: u16,
    body_len:   u32,
    client_ctx: u32,
    cas:        u64
}



impl PacketHeader {

    pub fn get_data_len(&mut self) -> uint {
        return self.body_len as uint - self.extra_len as uint - self.key_len as uint;
    }

    pub fn set_data_len(&mut self, data_len: uint) {
        self.body_len = (self.key_len as uint + self.extra_len as uint + data_len) as u32;
    }


    pub fn to_network(&mut self) {
        self.key_len = intrinsics::to_be16(self.key_len as i16) as u16;
        self.status_vbucket = intrinsics::to_be16(self.status_vbucket as i16) as u16;
        self.body_len = intrinsics::to_be32(self.body_len as i32) as u32;
        self.cas = intrinsics::to_be64(self.cas as i64) as u64;
    }

    pub fn pack(&self, buf: &mut [u8], mut offset: uint) -> uint {
        offset = ioutil::pack_u8_be(buf, offset, self.magic);
        offset = ioutil::pack_u8_be(buf, offset, self.opcode);
        offset = ioutil::pack_u16_be(buf, offset, self.key_len);
        offset = ioutil::pack_u8_be(buf, offset, self.extra_len);
        offset = ioutil::pack_u8_be(buf, offset, self.data_type);
        offset = ioutil::pack_u16_be(buf, offset, self.status_vbucket);
        offset = ioutil::pack_u32_be(buf, offset, self.body_len);
        offset = ioutil::pack_u32_be(buf, offset, self.client_ctx);
        offset = ioutil::pack_u64_be(buf, offset, self.cas);
        offset
    }

    pub fn unpack(&mut self, buf: &[u8], mut offset: uint) -> uint {
        self.magic = ioutil::unpack_u8_be(buf, offset);
        offset += 1;
        self.opcode = ioutil::unpack_u8_be(buf, offset);
        offset += 1;
        self.key_len = ioutil::unpack_u16_be(buf, offset);
        offset += 2;
        self.extra_len = ioutil::unpack_u8_be(buf, offset);
        offset += 1;
        self.data_type = ioutil::unpack_u8_be(buf, offset);
        offset += 1;
        self.status_vbucket = ioutil::unpack_u16_be(buf, offset);
        offset += 2;
        self.body_len = ioutil::unpack_u32_be(buf, offset);
        offset += 4;
        self.client_ctx = ioutil::unpack_u32_be(buf, offset);
        offset += 4;
        self.cas = ioutil::unpack_u64_be(buf, offset);
        offset += 8;
        offset
    }

}
