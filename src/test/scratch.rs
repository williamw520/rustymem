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


use std::int;



pub fn to_int(s : &str, default_value : int) -> int {
    match int::from_str(s) {
        Some(i) => i,
        None => default_value
    }
}

pub fn to_host_port(host_port_str: &str, default_port : int) -> (~str, u16) {
    let tokens = host_port_str
        .trim()
        .split_iter(':')
        .map(|s| s.trim())
        .collect::<~[&str]>();
    match tokens.len() {
        0 | 1   => (tokens[0].to_owned(), default_port as u16),
        _       => (tokens[0].to_owned(), to_int(tokens[1], default_port) as u16)
    }
}


// Test bed for trying things out.
fn main()  {

    println("scratch main() enter");

    let str1 = ~"this is a test ";
    let str1trim = str1.trim();
    println( fmt!("%?", str1) );
    println( fmt!("%?", str1trim) );
    let str1trim2 = str1.trim().to_owned();
    println( fmt!("%?", str1trim2) );

    let strs1 = str1.split_iter(' ').to_owned_vec();
    println( fmt!("%?", strs1) );

    let strs2 = ~[~"abc", ~"xyz"];
    println( fmt!("%?", strs2) );

    println( fmt!("%?", to_host_port(~"localhost: 9000", 1234)) );
    println( fmt!("%?", to_host_port(~"localhost:9000", 1234)) );
    println( fmt!("%?", to_host_port(~" localhost:9000", 1234)) );
    println( fmt!("%?", to_host_port(~"localhost : 9000 ", 1234)) );
    println( fmt!("%?", to_host_port(~"localhost ", 1234)) );
    println( fmt!("%?", to_host_port(~" localhost ", 1234)) );

    println("scratch main() exit");

}

