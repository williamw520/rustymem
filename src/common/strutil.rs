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


use std::str::CharEq;
//use std::int;
//use std::to_str;
//use std::from_str;


pub fn to_num<T: FromStr>(s : &str, default_value : T) -> T {
    match from_str::<T>(s.trim()) {
        Some(i) => i,
        None => default_value
    }
}

pub fn maybe_to_num<T: FromStr>(s : Option<~str>, default_value : T) -> T {
    match s {
        Some(s) => to_num(s, default_value),
        None => default_value
    }
}

// Split a string into parts, trimming it and trimming its parts after split.
pub fn clean_split<Sep: CharEq>(s : &str, seperator: Sep) -> ~[&str] {
    let parts = s
        .trim()
        .split_iter(seperator)
        .map(|p| p.trim())
        .collect::<~[&str]>();
    return parts;
}

#[test]
fn test_to_num() {
    println( fmt!("%?", from_str::<int>("28")) );
    println( fmt!("%?", to_num("9000", 1234)) );
    println( fmt!("%?", to_num("123", 1234)) );
    println( fmt!("%?", to_num("abc", 1234)) );
}

#[test]
fn test_clean_split() {
    println( fmt!("%?", clean_split("a.b.c", '.')) );
    println( fmt!("%?", clean_split("a.123.c", '.')) );
    println( fmt!("%?", clean_split(" a :123.c ", ':')) );
}

