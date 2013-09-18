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


use super::super::MemData;
use super::super::MemResponse;
use super::super::MemcachedStat;



/// Low level memcached protocol API
pub trait ProtoConnection {

    //// Storage commands

    // Store the data for the key.
    // Data never expires if exptime = 0, expires in seconds if between 1 to 60*60*24*30 (30 days), expires at the absolute Unix time if greater than 30 days.
    fn p_set(&mut self, key: &str, data: &[u8], flags: u32, exptime: uint, noreply: bool) -> MemResponse;

    // Store the data only if the server does not hold data for the key.
    fn p_add(&mut self, key: &str, data: &[u8], flags: u32, exptime: uint, noreply: bool) -> MemResponse;

    // Store the data only if the server does already hold data for the key.
    fn p_replace(&mut self, key: &str, data: &[u8], flags: u32, exptime: uint, noreply: bool) -> MemResponse;

    // Add the data after the existing data of the key.
    fn p_append(&mut self, key: &str, data: &[u8], noreply: bool) -> MemResponse;

    // Add the data before the existing data of the key.
    fn p_prepend(&mut self, key: &str, data: &[u8], noreply: bool) -> MemResponse;

    // Check and set.  Store the data only if it has not been updated since the last fetchd client, checking with the cas_unique value from last fetch.
    fn p_cas(&mut self, key: &str, data: &[u8], cas_unique: u64, flags: u32, exptime: uint, noreply: bool) -> MemResponse;


    //// Data command
    
    // Update the expiration time of an existing item without fetching it.
    fn p_touch(&mut self, key: &str, exptime: uint, noreply: bool) -> MemResponse;

    // Increment the existing 64-bit integer at the key by the inc_amount.
    fn p_incr(&mut self, key: &str, inc_amount: u64, noreply: bool) -> MemResponse;

    // Increment the existing 64-bit integer at the key by the inc_amount.
    fn p_incr_with(&mut self, key: &str, exptime: uint, inc_amount: u64, init_value: u64, noreply: bool) -> MemResponse;

    // Decrement the existing 64-bit integer at the key by the dec_amount.
    fn p_decr(&mut self, key: &str, dec_amount: u64, noreply: bool) -> MemResponse;

    fn p_decr_with(&mut self, key: &str, exptime: uint, dec_amount: u64, init_value: u64, noreply: bool) -> MemResponse;

    // Delete command
    fn p_delete(&mut self, key: &str, noreply: bool) -> MemResponse;


    //// Retrieval command

    // Retrieve multiple data at the corresponding keys.
    fn p_get(&mut self, key: &[&str]) -> Result<~[MemData], ~str>;

    // Retrieve multiple data at the corresponding keys.
    fn p_gets(&mut self, keys: &[&str]) -> Result<~[MemData], ~str>;


    //// Other commands

    // Get the version string of the server
    fn p_version(&mut self) -> Result<~str, ~str>;

    // Set the verbosity level of the logging output at the server
    fn p_verbosity(&mut self, verbosity: u32, noreply: bool) -> MemResponse;

    // Invalid all data at server.  Any subsequent retrieval by key will return no data.
    fn p_flush(&mut self, delay_in_seconds: uint, noreply: bool) -> MemResponse;

    // Return all server statistics
    fn p_stats(&mut self) -> Result<~[MemcachedStat], ~str>;

    // Server closes the connection from client.
    fn p_quit(&mut self) -> MemResponse;


    // Server config
    fn p_get_server_addr(&self) -> ~str;

}




