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
use std::vec;
use std::rt::io::net::tcp::TcpStream;
use std::rt::io::{Reader, Writer};


use common::strutil;
use common::netutil;


use super::super::MemData;
use super::super::MemResponse;
use super::super::MemcachedStat;
use super::super::No_Error;


use super::proto::ProtoConnection;


//
// ASCII Protocol
//


static CR: u8   = '\r' as u8;
static LF: u8   = '\n' as u8;
static SP: u8   = ' '  as u8;


/// Struct for one memcached server
struct AsciiConnection {
    server_addr:    ~netutil::HostAddr,
    stream:         Option<TcpStream>,
}

impl AsciiConnection {

    pub fn new_connection(server_addr: ~netutil::HostAddr) -> ~AsciiConnection {

        debug!("new_connection() enter");

        let stream = TcpStream::connect(server_addr.get_sock_addr());
        if stream.is_none() {
            fail!("connect() failed")
        }
        //debug!( fmt!("stream = %?", stream) );

        //let mut stream = BufferedStream::new(stream);
        //debug!( fmt!("bstream = %?", bstream) );

        // let bwriter = BufferedWriter::new(stream);
        // debug!( fmt!("bwriter = %?", bwriter) );

        // let breader = BufferedReader::new(stream);
        // debug!( fmt!("breader = %?", breader) );
        
        // self.writer = Some(self.stream as ~WriterUtil);
        // self.reader = Some(self.stream as ~ReaderUtil);

        //let reader = stream as ~std::rt::io::extensions::ReaderUtil;

        //let reader = ~stream as ~std::rt::io::Reader;
        //let writer = ~stream as ~std::rt::io::Writer;

        return ~AsciiConnection {
            server_addr:    server_addr,
            stream:         stream,
            // reader:         BufferedReader::new(stream),
            // writer:         ~stream as ~WriterUtil,
        };

    }

    fn ascii_get_server_addr(&self) -> ~str {
        return self.server_addr.to_str();
    }


    fn ascii_format_store_cmd(&self, cmd: &str, key: &str, data: &[u8], flags: u32, exptime: uint, noreply: bool) -> ~str {
        return format!("{} {} {} {} {} {}\r\n", cmd, key, flags, exptime, data.len(), (if noreply { "noreply" } else { "" }) );
    }

    fn ascii_format_cas_cmd(&self, key: &str, data: &[u8], cas_unique: u64, flags: u32, exptime: uint, noreply: bool) -> ~str {
        return format!("cas {} {} {} {} {} {}\r\n", key, flags, exptime, data.len(), cas_unique, (if noreply { "noreply" } else { "" }) );
    }

    fn ascii_send_store_request(&mut self, request: &str, data: &[u8], noreply: bool) -> MemResponse {
        debug!(request);
        self.ascii_write_data(request.as_bytes());
        self.ascii_write_data(data);
        self.ascii_write_data(bytes!("\r\n"));
        if noreply {
            No_Error
        } else {
            MemResponse::ascii_to_status(self.ascii_read_line())
        }
    }

    fn ascii_send_simple_request(&mut self, request: &str, noreply: bool) -> MemResponse {
        debug!(request);
        self.ascii_write_data(request.as_bytes());
        if noreply {
            No_Error
        } else {
            //return self.ascii_read_line();
            MemResponse::ascii_to_status(self.ascii_read_line())
        }
    }

    fn ascii_send_get_request(&mut self, request: &str) -> Result<~[MemData], ~str> {
        debug!(request);
        self.ascii_write_data(request.as_bytes());

        let mut mdata_list : ~[MemData] = ~[];
        let mut dummy = [0u8, ..2];
        loop {
            let value_line = self.ascii_read_line().unwrap();
            //debug!( fmt!("value_line: %?", value_line) );
            let tokens = strutil::clean_split(value_line, ' ');
            match tokens[0] {
                "VALUE" if tokens.len() >= 4  => {
                    let bytes = from_str::<u32>(tokens[3]).unwrap();
                    let mut mdata = MemData {
                        key:        tokens[1].to_owned(),
                        flags:      from_str::<u32>(tokens[2]).unwrap(),
                        cas_unique: if tokens.len() >= 5 { from_str::<u64>(tokens[4]).unwrap() } else { 0u64 },
                        data:       vec::from_elem(bytes as uint, 0u8)
                    };
                    self.stream.read(mdata.data);
                    self.stream.read(dummy);
                    //debug!( fmt!("mdata: %?", mdata) );
                    mdata_list.push(mdata);
                },
                "END" => {
                    break;
                },
                _ => return Err( fmt!("Bad response: %?", value_line) )
            }
        }
        return Ok(mdata_list);
    }

    fn ascii_send_stats_request(&mut self, request: &str) -> Result<~[MemcachedStat], ~str> {
        debug!(request);
        self.ascii_write_data(request.as_bytes());

        let mut stats : ~[MemcachedStat] = ~[];
        loop {
            let stat_line = self.ascii_read_line().unwrap();
            //debug!( fmt!("stat_line: %?", stat_line) );
            let tokens = strutil::clean_split(stat_line, ' ');
            match tokens[0] {
                "STAT" if tokens.len() >= 3  => {
                    stats.push(MemcachedStat {
                            name:   tokens[1].to_owned(),
                            value:  tokens[2].to_owned()
                        });
                },
                "END" => {
                    break;
                },
                _ => return Err( fmt!("Bad response: %?", stat_line) )
            }
        }
        return Ok(stats);
    }


    fn ascii_write_data(&mut self, data: &[u8]) {
        self.stream.write(data);
    }

    fn ascii_read_line(&mut self) -> Result<~str, ~str> {
        // TODO: terribly inefficient.  Replace this with Rust's BufferedStream version when it's ready. 
        let mut line = ~"";
        let mut buf = [0u8, ..1];
        loop {
            self.stream.read(buf);
            match buf[0] {
                CR => {
                    self.stream.read(buf);
                    if buf[0] == LF {
                        break;
                    } else {
                        return Err(~"Missing LF after CR from server");
                    }
                },
                _ => {
                    line.push_char(buf[0] as char);
                }
            }
        }
        return Ok(line);
    }

}



// ProtoConnection implementation for one memcached server
impl ProtoConnection for AsciiConnection {

    //// Storage commands

    fn p_set(&mut self,  key: &str,  data: &[u8],  flags: u32,  exptime: uint,  noreply: bool) -> MemResponse {
        let req = self.ascii_format_store_cmd("set", key, data, flags, exptime, noreply);
        return self.ascii_send_store_request(req, data, noreply);
    }

    fn p_add(&mut self,  key: &str,  data: &[u8],  flags: u32,  exptime: uint,  noreply: bool) -> MemResponse {
        let req = self.ascii_format_store_cmd("add", key, data, flags, exptime, noreply);
        return self.ascii_send_store_request(req, data, noreply);
    }

    fn p_replace(&mut self,  key: &str,  data: &[u8],  flags: u32,  exptime: uint,  noreply: bool) -> MemResponse {
        let req = self.ascii_format_store_cmd("replace", key, data, flags, exptime, noreply);
        return self.ascii_send_store_request(req, data, noreply);
    }

    fn p_append(&mut self,  key: &str,  data: &[u8],  noreply: bool) -> MemResponse {
        // flags and exptime are ignored by the server
        let req = self.ascii_format_store_cmd("append", key, data, 0, 0, noreply);
        return self.ascii_send_store_request(req, data, noreply);
    }

    fn p_prepend(&mut self,  key: &str,  data: &[u8],  noreply: bool) -> MemResponse {
        // flags and exptime are ignored by the server
        let req = self.ascii_format_store_cmd("prepend", key, data, 0, 0, noreply);
        return self.ascii_send_store_request(req, data, noreply);
    }

    fn p_cas(&mut self,  key: &str,  data: &[u8],  cas_unique: u64,  flags: u32,  exptime: uint,  noreply: bool) -> MemResponse {
        let req = self.ascii_format_cas_cmd(key, data, cas_unique, flags, exptime, noreply);
        return self.ascii_send_store_request(req, data, noreply);
    }


    //// Data command
    
    fn p_touch(&mut self, key: &str, exptime: uint, noreply: bool) -> MemResponse {
        let req = format!("touch {} {} {}\r\n", key, exptime, (if noreply { "noreply" } else { "" }) );
        return self.ascii_send_simple_request(req, noreply);
    }

    fn p_incr(&mut self, key: &str, inc_amount: u64, noreply: bool) -> MemResponse {
        let req = format!("incr {} {} {}\r\n", key, inc_amount, (if noreply { "noreply" } else { "" }) );
        return self.ascii_send_simple_request(req, noreply);
    }

    fn p_incr_with(&mut self, key: &str, exptime: uint, inc_amount: u64, init_value: u64, noreply: bool) -> MemResponse {
        // TODO: if response is NOT_FOUND, call p_set with init_value, return the init_value
        let req = format!("incr {} {} {}\r\n", key, inc_amount, (if noreply { "noreply" } else { "" }) );
        return self.ascii_send_simple_request(req, noreply);
    }

    fn p_decr(&mut self, key: &str, dec_amount: u64, noreply: bool) -> MemResponse {
        let req = format!("decr {} {} {}\r\n", key, dec_amount, (if noreply { "noreply" } else { "" }) );
        return self.ascii_send_simple_request(req, noreply);
    }

    fn p_decr_with(&mut self, key: &str, exptime: uint, dec_amount: u64, init_value: u64, noreply: bool) -> MemResponse {
        // TODO: if response is NOT_FOUND, call p_set with init_value, return the init_value
        let req = format!("decr {} {} {}\r\n", key, dec_amount, (if noreply { "noreply" } else { "" }) );
        return self.ascii_send_simple_request(req, noreply);
    }


    fn p_delete(&mut self, key: &str, noreply: bool) -> MemResponse {
        let req = format!("delete {} {}\r\n", key, (if noreply { "noreply" } else { "" }) );
        return self.ascii_send_simple_request(req, noreply);
    }


    //// Retrieval command

    fn p_get(&mut self, keys: &[&str]) -> Result<~[MemData], ~str> {
        let req = "get " + keys.connect(" ") + "\r\n";
        return self.ascii_send_get_request(req);
    }

    fn p_gets(&mut self, keys: &[&str]) -> Result<~[MemData], ~str> {
        let req = "gets " + keys.connect(" ") + "\r\n";
        return self.ascii_send_get_request(req);
    }


    //// Other commands

    fn p_version(&mut self) -> Result<~str, ~str> {
        self.ascii_write_data(bytes!("version\r\n"));
        return self.ascii_read_line();
    }

    fn p_verbosity(&mut self, verbosity: u32, noreply: bool) -> MemResponse {
        let req = format!("verbosity {} {}\r\n", verbosity, (if noreply { "noreply" } else { "" }) );
        return self.ascii_send_simple_request(req, noreply);
    }

    fn p_flush(&mut self, delay_in_seconds: uint, noreply: bool) -> MemResponse {
        let req = format!("flush_all {} {}\r\n", delay_in_seconds, (if noreply { "noreply" } else { "" }) );
        return self.ascii_send_simple_request(req, noreply);
    }

    fn p_stats(&mut self) -> Result<~[MemcachedStat], ~str> {
        return self.ascii_send_stats_request(("stats\r\n"));
    }

    fn p_quit(&mut self) -> MemResponse {
        return self.ascii_send_simple_request( "quit\r\n", false );
    }


    // Server config
    fn p_get_server_addr(&self) -> ~str {
        return self.server_addr.to_str();
    }


}



