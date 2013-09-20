#[link(name = "rustymem",
       vers = "0.1",
       uuid = "d491af7a-32d3-48dc-9507-d2c9fbd1263b")];
#[crate_type = "lib"];


/******************************************************************************
 * RustyMem, a Memcached client library in Rust.
 */


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


extern mod extra;

use std::result::Result;
use std::str;
use std::to_str::ToStr;
use std::vec;
use extra::json;
use extra::json::Json;
use extra::json::ToJson;
use extra::md5::Md5;
use extra::digest::Digest;

use common::strutil;
use common::netutil;
use common::ioutil;
use rustymem_lib::ascii_conn::AsciiConnection;
use rustymem_lib::binary_conn::BinaryConnection;

// Re-export
pub use rustymem_lib::proto::ProtoConnection;


// Configure the modules in this crate
mod rustymem_lib {
    pub mod proto;
    pub mod ascii_conn;
    pub mod binary_conn;
}
mod common {
    pub mod apputil;
    pub mod strutil;
    pub mod netutil;
    pub mod ioutil;
}




/// Constants
static DEFAULT_PORT : u16   = 11211u16; // default Memcached server port



/// Create a new RustyMem, passing in one server address or a list of servers for cluster.
/// connect("127.0.0.1");
/// connect("127.0.0.1:11211");
/// connect("127.0.0.1 127.0.0.2:11212 127.0.0.3:11213");
pub fn connect(server_addrs: &str) -> RustyMem  {
    // defaul to use the newer binary protocol.
    return connect_with( MemParams { servers: server_addrs.to_owned(), protocol: P_BINARY, shard: HASH_MOD } );
}


/// Create a new RustyMem, passing in one server address or a list of servers for cluster.
/// Pass in the Memcached protocol to use.  Note: all servers need to support the same protocol.
/// connect_with( MemParams { servers: ~"127.0.0.1", protocol: P_BINARY, shard: HASH_MOD } )
pub fn connect_with(params: MemParams) -> RustyMem  {
    debug!( fmt!("connect_with() enter, %?", params) );

    let addrs = strutil::clean_split(params.servers, ' ');
    let connections = addrs.iter().map( |addr| new_protocol_connection(*addr, params.protocol) ).collect::<~[~ProtoConnection]>();
    let conn_addrs = connections.iter().map( |conn| conn.p_get_server_addr() ).collect::<~[~str]>();
    debug!( fmt!("server_addrs : %?", conn_addrs) );

    RustyMem {
        params: params,
        connections: connections,
    }
}

fn new_protocol_connection(server_addr: &str, protocol: MemProtocol) -> ~ProtoConnection {
    let host_addr = netutil::HostAddr::with_host_port(server_addr, DEFAULT_PORT);
    match protocol {
        // TODO: collect connection errors and save them in RustyMem
        P_ASCII     => ~AsciiConnection::new_connection(host_addr) as ~ProtoConnection,
        P_BINARY    => ~BinaryConnection::new_connection(host_addr) as ~ProtoConnection,
    }
}


pub struct RustyMem {
    params:         MemParams,
    connections:    ~[~ProtoConnection]
}

/// Main entry for the Memcached API
impl RustyMem {

    /// Set data bytes at key in memcached, with the expiration exptime in seconds.  Setting exptime to 0 for no expiration.
    pub fn set_bytes(&mut self, key: &str, exptime: uint, data_bytes: &[u8]) -> MemResult<u64> {
        return self.conn(key).p_set(key, data_bytes, 0, 0, exptime, false);
    }

    /// Set data str at key in memcached, with the expiration exptime in seconds.  Setting exptime to 0 for no expiration.
    pub fn set_str(&mut self, key: &str, exptime: uint, data_str: &str) -> MemResult<u64> {
        return self.conn(key).p_set(key, data_str.as_bytes(), 0, 0, exptime, false);
    }

    /// Set data value as string at key in memcached, with the expiration exptime in seconds.  Setting exptime to 0 for no expiration.
    /// Set value can be retrieved with get_as().
    pub fn set_as<T: ToStr>(&mut self, key: &str, exptime: uint, value: &T) -> MemResult<u64> {
        return self.set_str(key, exptime, value.to_str());
    }

    /// Set data value as JSON string at key in memcached, with the expiration exptime in seconds.  Setting exptime to 0 for no expiration.
    pub fn set_json<T: ToJson>(&mut self, key: &str, exptime: uint, data_json: &T) -> MemResult<u64> {
        let json_str = data_json.to_json().to_str();
        return self.conn(key).p_set(key, json_str.as_bytes(), 0, 0, exptime, false);
    }


    /// Check and set data bytes at key in memcached, with the expiration exptime in seconds.  Setting exptime to 0 for no expiration.
    /// Pass in the last retrieved MemData.cas to check.
    pub fn cas_bytes(&mut self, key: &str, cas: u64, exptime: uint, data_bytes: &[u8]) -> MemResult<u64> {
        return self.conn(key).p_cas(key, data_bytes, cas, 0, exptime, false);
    }

    /// Check and set data str at key in memcached, with the expiration exptime in seconds.  Setting exptime to 0 for no expiration.
    /// Pass in the last retrieved MemData.cas to check.
    pub fn cas_str(&mut self, key: &str, cas: u64, exptime: uint, data_str: &str) -> MemResult<u64> {
        return self.conn(key).p_cas(key, data_str.as_bytes(), cas, 0, exptime, false);
    }

    /// Check and set data value as string at key in memcached, with the expiration exptime in seconds.  Setting exptime to 0 for no expiration.
    /// Pass in the last retrieved MemData.cas to check.
    /// Set value can be retrieved with get_as().
    pub fn cas_as<T: ToStr>(&mut self, key: &str, cas: u64, exptime: uint, value: &T) -> MemResult<u64> {
        return self.cas_str(key, cas, exptime, value.to_str());
    }

    /// Check and set data value as JSON string at key in memcached, with the expiration exptime in seconds.  Setting exptime to 0 for no expiration.
    /// Pass in the last retrieved MemData.cas to check.
    pub fn cas_json<T: ToJson>(&mut self, key: &str, cas: u64, exptime: uint, data_json: &T) -> MemResult<u64> {
        let json = data_json.to_json();
        let json_str = json.to_str();
        return self.conn(key).p_cas(key, json_str.as_bytes(), cas, 0, exptime, false);
    }


    pub fn add_bytes(&mut self, key: &str, exptime: uint, data_bytes: &[u8]) -> MemResult<u64> {
        return self.conn(key).p_add(key, data_bytes, 0, 0, exptime, false);
    }

    pub fn add_str(&mut self, key: &str, exptime: uint, data_str: &str) -> MemResult<u64> {
        return self.conn(key).p_add(key, data_str.as_bytes(), 0, 0, exptime, false);
    }

    pub fn add_as<T: ToStr>(&mut self, key: &str, exptime: uint, value: &T) -> MemResult<u64> {
        return self.add_str(key, exptime, value.to_str());
    }

    pub fn add_json<T: ToJson>(&mut self, key: &str, exptime: uint, data_json: &T) -> MemResult<u64> {
        let json_str = data_json.to_json().to_str();
        return self.conn(key).p_add(key, json_str.as_bytes(), 0, 0, exptime, false);
    }


    pub fn replace_bytes(&mut self, key: &str, cas: u64, exptime: uint, data_bytes: &[u8]) -> MemResult<u64> {
        return self.conn(key).p_replace(key, data_bytes, cas, 0, exptime, false);
    }

    pub fn replace_str(&mut self, key: &str, cas: u64, exptime: uint, data_str: &str) -> MemResult<u64> {
        return self.conn(key).p_replace(key, data_str.as_bytes(), cas, 0, exptime, false);
    }

    pub fn replace_as<T: ToStr>(&mut self, key: &str, cas: u64, exptime: uint, value: &T) -> MemResult<u64> {
        return self.replace_str(key, cas, exptime, value.to_str());
    }

    pub fn replace_json<T: ToJson>(&mut self, key: &str, cas: u64, exptime: uint, data_json: &T) -> MemResult<u64> {
        let json_str = data_json.to_json().to_str();
        return self.conn(key).p_replace(key, json_str.as_bytes(), cas, 0, exptime, false);
    }


    pub fn append_bytes(&mut self, key: &str, data_bytes: &[u8]) -> MemResult<u64> {
        return self.conn(key).p_append(key, data_bytes, false);
    }

    pub fn prepend_bytes(&mut self, key: &str, data_bytes: &[u8]) -> MemResult<u64> {
        return self.conn(key).p_prepend(key, data_bytes, false);
    }


    /// Get the data item as MemData at key from memcached.  Return None if no data found or error.
    /// MemData has all the info about the data item.
    pub fn get_data(&mut self, key: &str) -> Option<MemData> {
        let md_list = self.conn(key).p_gets([key]);
        if md_list.len() == 0 {
            None
        } else {
            Some(md_list[0])
        }
    }

    /// Get data bytes at key from memcached.  Return None if no data found or error.
    pub fn get_bytes(&mut self, key: &str) -> Option<~[u8]> {
        match self.get_data(key) {
            Some(md) => Some(md.as_bytes()),
            None => None
        }
    }

    /// Get data str at key from memcached.  Return None if no data found or error.
    pub fn get_str(&mut self, key: &str) -> Option<~str> {
        match self.get_data(key) {
            Some(md) => Some(md.as_str()),
            None => None
        }
    }

    /// Get data value as type from string at key from memcached.  Return None if no data found or error.
    pub fn get_as<T: FromStr>(&mut self, key: &str) -> Option<T> {
        match self.get_data(key) {
            Some(md) => md.as_type::<T>(),
            None => None
        }
    }

    /// Get Json at key from memcached.  Return None if no data found or error.
    pub fn get_json(&mut self, key: &str) -> Option<Json> {
        match self.get_data(key) {
            Some(md) => match md.as_json() {
                Ok(j) => Some(j),
                Err(_) => None
            },
            None => None
        }
    }


    /// Get the list of data as MemData of the list of keys.  Return empty list if no data found or error.
    pub fn get_bulk_data(&mut self, keys: &[&str]) -> ~[MemData] {
        if self.get_connection_count() == 1 {
            let conn = self.get_connection(0);
            return conn.p_gets(keys);
        }

        let mut result : ~[MemData] = ~[];
        let key_arrays : ~[~[~str]] = RustyMem::distribute_keys(keys, self.get_connection_count(), RustyMem::md5_mod_indexer);
        for i in range(0, self.get_connection_count()) {
            if key_arrays[i].len() > 0 {
                let key_array : &~[~str] = &key_arrays[i];
                let key_ref_array : ~[&str] = key_array.iter().map(|k| k.as_slice()).to_owned_vec();
                let ref_of_key_ref_array : &[&str] = key_ref_array.as_slice();
                let conn = self.get_connection(i);
                let conn_result = conn.p_gets(ref_of_key_ref_array);
                result.push_all_move(conn_result);
            }
        }
        return result;

    }

    /// Get the list of data as bytes of the list of keys.  Return empty list if no data found or error.
    pub fn get_bulk_bytes(&mut self, keys: &[&str]) -> ~[(~str, ~[u8])] {
        let md_list = self.get_bulk_data(keys);
        return md_list.iter().map(|md| ( md.key.clone(), md.as_bytes() ) ).collect::<~[(~str, ~[u8])]>();
    }

    /// Get the list of data as str of the list of keys.  Return empty list if no data found or error.
    pub fn get_bulk_str(&mut self, keys: &[&str]) -> ~[(~str, ~str)] {
        let md_list = self.get_bulk_data(keys);
        return md_list.iter().map(|md| ( md.key.clone(), md.as_str() ) ).collect::<~[(~str, ~str)]>();
    }

    /// Get the list of data value from string of the list of keys.  Return empty list if no data found or error.
    pub fn get_bulk_as<T: FromStr>(&mut self, keys: &[&str]) -> ~[(~str, Option<T>)] {
        let md_list = self.get_bulk_data(keys);
        return md_list.iter().map(|md| ( md.key.clone(), md.as_type::<T>() ) ).collect::<~[(~str, Option<T>)]>();
    }

    /// Get the list of data as Json of the list of keys.  Return empty list if no data found or error.
    pub fn get_bulk_json(&mut self, keys: &[&str]) -> ~[(~str, Result<Json, json::Error>)] {
        let md_list = self.get_bulk_data(keys);
        return md_list.iter().map(|md| ( md.key.clone(), md.as_json() ) ).collect::<~[(~str, Result<Json, json::Error>)]>();
    }


    // Data Functions

    /// Update a cached entry's expiration time.  If entry exists, return TOUCHED.  If entry not exists, return NOT_FOUND.
    /// Note: touch command is not supported in the current Memcached version.
    pub fn touch(&mut self, key: &str, exptime: uint) -> MemStatus {
        return self.conn(key).p_touch(key, exptime, false);
    }

    pub fn delete(&mut self, key: &str) -> MemStatus {
        return self.conn(key).p_delete(key, false);
    }

    // Increment the existing 64-bit integer at the key by the inc_amount.
    pub fn incr(&mut self, key: &str, inc_amount: u64, init_value: u64, exptime: uint) -> MemResult<u64> {
        return self.conn(key).p_incr(key, inc_amount, init_value, exptime, false);
    }

    // Decrement the existing 64-bit integer at the key by the dec_amount.
    pub fn decr(&mut self, key: &str, dec_amount: u64, init_value: u64, exptime: uint) -> MemResult<u64> {
        return self.conn(key).p_decr(key, dec_amount, init_value, exptime, false);
    }


    pub fn flush(&mut self, delay_in_seconds: uint) -> ~[MemStatus] {
        return self.connections.mut_iter().map( |conn| {
                conn.p_flush(delay_in_seconds, false)
            } ).collect::<~[MemStatus]>();
    }

    pub fn verbosity(&mut self, verbosity: u32) -> ~[MemStatus] {
        return self.connections.mut_iter().map( |conn| {
                conn.p_verbosity(verbosity, false)
            } ).collect::<~[MemStatus]>();
    }

    pub fn stats(&mut self) -> ~[~[MemcachedStat]] {
        return self.connections.mut_iter().map( |conn| {
                conn.p_stats()
            } ).collect::<~[~[MemcachedStat]]>();
    }

    pub fn quit(&mut self) -> ~[MemStatus] {
        return self.connections.mut_iter().map( |conn| {
                conn.p_quit()
            } ).collect::<~[MemStatus]>();
    }


    pub fn versions(&mut self) -> ~[~str] {
        return self.connections.mut_iter().map( |conn| {
                match conn.p_version() {
                    Ok(v)  => v,
                    Err(e) => e
                }
            } ).collect::<~[~str]>();
    }

    pub fn get_connection_count(&self) -> uint {
        return self.connections.len();
    }

    pub fn get_connection<'a>(&'a mut self, index: uint) -> &'a mut ~ProtoConnection {
        return &mut self.connections[index];
    }


    // Pick a connection based on key value.  Simple hash % N algorithm for now.
    fn conn<'r>(&'r mut self, key: &str) -> &'r mut ~ProtoConnection {
        let mut index;
        if self.get_connection_count() == 1 {
            index = 0;
        } else {
            // TODO: check self.params.shard
            index = RustyMem::md5_mod_indexer(key, self.connections.len());
        }
        return &mut self.connections[index];
    }

    // Compute connection index of a key based on md5(key) mod connection.len()
    fn md5_mod_indexer(key: &str, connection_count: uint) -> uint {
        if connection_count == 1 {
            return 0;
        }

        let mut result = vec::from_elem(16, 0u8);
        let mut digest = Md5::new();
        digest.input(key.as_bytes());
        digest.result(result);
        let val4 = ioutil::trunc_bytes(result) as uint;
        return val4 % connection_count;
    }

    // Distribute the keys to N partitions according to its indexer function.
    // Build a key array for each partition for the belonging keys.
    fn distribute_keys(keys: &[&str], partition_count: uint, indexer: &fn(&str, uint)->uint) -> ~[~[~str]] {
        let mut array_of_keys_for_partition : ~[~[~str]] = vec::from_elem::<~[~str]>(partition_count, ~[]);

        // if partition_count == 1 {
        //     let array_at_index = &mut array_of_keys_for_partition[0];
        //     for i in range(0, keys.len()) {
        //         let key = keys[i].to_owned();
        //     array_at_index.push(key);
        // }

        let connection_index_of_key : ~[uint] = keys.iter().map(|key| indexer(*key, partition_count)).collect::<~[uint]>();
        for i in range(0, partition_count) {
            let index = connection_index_of_key[i];
            let key = keys[i].to_owned();
            let array_at_index = &mut array_of_keys_for_partition[index];
            array_at_index.push(key);
        }

        return array_of_keys_for_partition;
    }

}


//
// Public defs
//


pub struct MemParams {
    servers:    ~str,
    protocol:   MemProtocol,
    shard:      ShardMethod
}

pub enum MemProtocol {
    /// Use Memcached ASCII protocol
    P_ASCII,
    /// Use Memcached binary protocol
    P_BINARY,
}

pub enum ShardMethod {
    /// Use MD5 to hash key then mod by the server count.
    HASH_MOD,
}


/// Response codes of Memcached calls
pub enum MemStatus {
    // Ok
    Success = 0x0000,

    // Server error
    Key_Not_Found = 0x0001,
    Key_Exists = 0x0002,
    Value_Too_Large = 0x0003,
    Invalid_Arguments = 0x0004,
    Item_Not_Stored = 0x0005,
    Non_Numeric_Value = 0x0006,
    Vbucket_Belongs_Another_Server = 0x0007,
    Authentication_Error = 0x0008,
    Authentication_Continue = 0x0009,
    Unknown_Command = 0x0081,
    Out_Of_Memory = 0x0082,
    Not_Supported = 0x0083,
    Internal_Error = 0x0084,
    Busy = 0x0085,
    Temporary_Failure = 0x0086,

    // Custom errors
    Network_Error = 0x0200,
    Unknown_Response = 0x0201,
    Not_Implemented = 0x0202,
}

impl MemStatus {
    pub fn ascii_to_status(r : Result<~str, ~str>) -> MemStatus {
        match r {
            Ok(s)   => { 
                    let tokens = strutil::clean_split(s, ' ');  
                    MemStatus::map_ascii_status(tokens[0])   
            },
            Err(_)  => Network_Error
        }
    }

    fn map_ascii_status(response_token: &str) -> MemStatus {
        match response_token {
            "OK"            => Success,
            "STORED"        => Success,
            "NOT_STORED"    => Item_Not_Stored,
            "EXISTS"        => Key_Exists,
            "NOT_FOUND"     => Key_Not_Found,
            "DELETED"       => Success,
            "TOUCHED"       => Success,
            "ERROR"         => Unknown_Command,
            "CLIENT_ERROR"  => Invalid_Arguments,
            "SERVER_ERROR"  => Internal_Error,
            _               => Unknown_Response
        }
    }

    pub fn map_status(status: u16) -> MemStatus {
        match status {
            0x0000 => Success,
            0x0001 => Key_Not_Found,
            0x0002 => Key_Exists,
            0x0003 => Value_Too_Large,
            0x0004 => Invalid_Arguments,
            0x0005 => Item_Not_Stored,
            0x0006 => Non_Numeric_Value,
            0x0007 => Vbucket_Belongs_Another_Server,
            0x0008 => Authentication_Error,
            0x0009 => Authentication_Continue,
            0x0081 => Unknown_Command,
            0x0082 => Out_Of_Memory,
            0x0083 => Not_Supported,
            0x0084 => Internal_Error,
            0x0085 => Busy,
            0x0086 => Temporary_Failure,

            0x0200 => Network_Error,
            0x0201 => Unknown_Response,
            0x0202 => Not_Implemented,

            _ => Unknown_Response
        }
    }
    
}


pub struct MemResult<T> {
    status:     MemStatus,
    value:      T
}

/// The returned result of the Get query from Memcached.
pub struct MemData {
    /// Key of the returned data
    key:        ~str,
    /// The returned data
    data:       ~[u8],
    /// The CAS value for the next cas operation to ensure no one has changed the data in the memcached server
    cas:        u64,
    /// Flags associated with the data.
    flags:      u32
}

impl MemData {
    /// Return pointer to the retrieved data bytes.
    pub fn as_data_ptr<'a>(&'a self) -> &'a ~[u8] {
        return &self.data;
    }

    // Return the retrieved data as cloned bytes
    pub fn as_bytes(&self) -> ~[u8] {
        return self.data.clone();
    }

    /// Return the retrieved data as str
    pub fn as_str(&self) -> ~str {
        return str::from_utf8(self.data);
    }

    /// Return the retrieved data as Json
    pub fn as_json(&self) -> Result<Json, json::Error> {
        return json::from_str(self.as_str());
    }

    /// Convert the return data string into any type that can converted from FromStr.
    /// e.g. as_type::<int>(), as_type::<bool>()
    pub fn as_type<T: FromStr>(&self) -> Option<T> {
        // Clean up string
        let s = self.as_str();
        let ts = s.trim();
        let ts2 = ts.trim_chars(&(0u8 as char));
        return from_str::<T>(ts2);
    }

    /// Convert the return data string into any type that can converted from FromStr.
    /// Return the default value if conversion failed.
    pub fn as_type_of<T: FromStr>(&self, default_value : T) -> T {
        match self.as_type::<T>() {
            Some(value) => value,
            None => default_value
        }
    }

}

impl ToStr for MemData {
    fn to_str(&self) -> ~str {
        return str::from_utf8(self.data);
    }
}



/// Stat result entry for the stats query
pub struct MemcachedStat {
    name:       ~str,
    value:      ~str
}

