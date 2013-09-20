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


use std::rt::io::net::ip::IpAddr;
use std::rt::io::net::ip::Ipv4Addr;
use std::rt::io::net::ip::SocketAddr;


use super::strutil;



/// Parse the "host:port" string, with support for default_port.
pub fn to_host_port(host_port_str: &str, default_port : u16) -> (~str, u16) {
    let tokens = strutil::clean_split(host_port_str, ':');
    match tokens.len() {
        0   => fail!( fmt!("Fail to %?", host_port_str) ),
        1   => (tokens[0].to_owned(), default_port),
        _   => (tokens[0].to_owned(), strutil::to_num(tokens[1], default_port))
    }
}



/// Structure for holding, parsing, and formating hostname and port net address.
pub struct HostAddr {
    /// The hostname or ip address
    host:       ~str,

    /// The port number
    port:       Option<u16>,
}

impl HostAddr {

    pub fn new() -> HostAddr {
        return HostAddr {
            host: "".to_owned(),
            port: None
        };
    }

    /// Create one from host:port string
    pub fn with_host_port(host_port_str: &str, default_port : u16) -> HostAddr {
        let (host, port) = to_host_port(host_port_str, default_port);
        return HostAddr {
            host: host,
            port: Some(port)
        };
    }

    pub fn get_host(&self) -> ~str {
        self.host.clone()
    }

    pub fn get_port(&self) -> u16 {
        match self.port {
            Some(port) => port,
            None => 0
        }
    }

    pub fn get_ip_v4(&self) -> IpAddr {
        let parts = strutil::clean_split(self.host, '.').map(|s| strutil::to_num(*s, 0u8));
        return Ipv4Addr(parts[0], parts[1], parts[2], parts[3]);
    }

    pub fn get_sock_addr(&self) -> SocketAddr {
        return SocketAddr { 
                ip: self.get_ip_v4(), 
                port: self.get_port() 
        };
    }


}

impl ToStr for HostAddr {
    fn to_str(&self) -> ~str {
        match self.port {
            Some(port) => fmt!("%s:%s", self.host, port.to_str()),
            None => self.host.clone(),
        }
    }
}



#[test]
fn test_to_host_port()  {

    println( fmt!("%?", to_host_port("localhost: 9000", 1234)) );
    println( fmt!("%?", to_host_port("localhost:9000", 1234)) );
    println( fmt!("%?", to_host_port(" localhost:9000", 1234)) );
    println( fmt!("%?", to_host_port(" localhost : 9000 ", 1234)) );
    println( fmt!("%?", to_host_port("localhost : 9000 ", 1234)) );
    println( fmt!("%?", to_host_port("localhost ", 1111)) );
    println( fmt!("%?", to_host_port(" localhost ", 2222)) );
    println( fmt!("%?", to_host_port(" localhost: ", 3333)) );
    println( fmt!("%?", to_host_port(" localhost:abc ", 4444)) );

}

