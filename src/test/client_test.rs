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
extern mod rustymem;

//use extra::json::ToJson;    // need this for set_json
use std::str;
use std::vec;
use std::hashmap::HashMap;

use rustymem::*;
use rustymem::ProtoConnection;


fn test_new_conn() -> ~RustyMem {
    rustymem::new_with_protocol("127.0.0.1", P_ASCII)
}

fn test_binary_conn() {

    let mut rm = rustymem::new_with_protocol("127.0.0.1:11211", P_BINARY);

    println( fmt!("versions: %?", rm.versions()) );

    println( fmt!("set_bytes key1 key1value: %?", rm.set_bytes("key1", 2*60*60, bytes!("key1value"))) );

    println( fmt!("add_bytes key1 key1value: %?", rm.add_bytes("key1", 2*60*60, bytes!("key1value"))) );
    println( fmt!("add_bytes key2 key2value: %?", rm.add_bytes("key2", 2*60*60, bytes!("key2value"))) );

    println( fmt!("set_bytes key3 key1value: %?", rm.set_bytes("key3", 2*60*60, bytes!("key3value"))) );

    println( fmt!("replace_bytes key1 key1valueABC: %?", rm.replace_bytes("key1", 0, 60, bytes!("key1valueABC"))) );
    println( fmt!("replace_bytes key_none key3value: %?", rm.replace_bytes("key_none", 0, 60, bytes!("key3value"))) );

    println( fmt!("append_bytes key1 xyz: %?", rm.append_bytes("key1", bytes!("xyz"))) );
    println( fmt!("append_bytes key_none xyz: %?", rm.append_bytes("key_none", bytes!("xyz"))) );

    println( fmt!("prepend_bytes key1 123: %?", rm.prepend_bytes("key1", bytes!("123"))) );
    println( fmt!("prepend_bytes key_none 123: %?", rm.prepend_bytes("key_none", bytes!("123"))) );

    println( fmt!("delete key1 : %?", rm.delete("key1")) );
    println( fmt!("delete key_none : %?", rm.delete("key_none")) );

    println( fmt!("touch key1 : %?", rm.touch("key1", 10)) );

    println( fmt!("verbosity : %?", rm.verbosity(10)) );

    println( fmt!("stats : %?", rm.stats()) );

    println( fmt!("get_str key1: %?", rm.get_str("key1")) );

    println( fmt!("get_bulk_str key1 key2 key3 key_none: %?", rm.get_bulk_str(["key1", "key2", "key3", "key_none"])) );
    println( fmt!("get_bulk_str key1 key2 key_none key3: %?", rm.get_bulk_str(["key1", "key2", "key_none", "key3"])) );
    println( fmt!("get_bulk_str key_none key_none: %?", rm.get_bulk_str(["key_none", "key_none", "key_none", "key_none"])) );

    //println( fmt!("quit : %?", rm.quit()) );
    //println( fmt!("versions: %?", rm.versions()) );

}


fn test_cluster() {

    let mut rm = rustymem::new_with_protocol("127.0.0.1:11211 127.0.0.1:11212", P_BINARY);

    println( fmt!("versions: %?", rm.versions()) );

    println( fmt!("set_bytes key1: %?", rm.set_bytes("key1", 60, bytes!("key1value"))) );
    println( fmt!("set_bytes key2: %?", rm.set_bytes("key2", 60, bytes!("key2value"))) );
    println( fmt!("set_bytes key3: %?", rm.set_bytes("key3", 60, bytes!("key3value"))) );
    println( fmt!("set_bytes key4: %?", rm.set_bytes("key4", 60, bytes!("key4value"))) );
    println( fmt!("set_bytes key5: %?", rm.set_bytes("key5", 60, bytes!("key5value"))) );
    println( fmt!("set_bytes key6: %?", rm.set_bytes("key6", 60, bytes!("key6value"))) );
    // get key1 key2 key3 key4 key5 key6

    println( fmt!("get_bulk_str key1 key2 key3 key_none: %?", rm.get_bulk_str(["key1", "key2", "key3", "key_none"])) );
    println( fmt!("get_bulk_str key1 key2 key_none key3: %?", rm.get_bulk_str(["key1", "key2", "key_none", "key3"])) );
    println( fmt!("get_bulk_str key_none key_none: %?", rm.get_bulk_str(["key_none", "key_none", "key_none", "key_none"])) );

    println( fmt!("stats : %?", rm.stats()) );

    println( fmt!("flush: %?", rm.flush(0)) );

    println( fmt!("replace_bytes key1 key1valueABC: %?", rm.replace_bytes("key1", 0, 60, bytes!("key1valueABC"))) );

    println( fmt!("get_bulk_str key1 key2 key3 key_none: %?", rm.get_bulk_str(["key1", "key2", "key3", "key_none"])) );
    println( fmt!("get_bulk_str key1 key2 key_none key3: %?", rm.get_bulk_str(["key1", "key2", "key_none", "key3"])) );
    println( fmt!("get_bulk_str key_none key_none: %?", rm.get_bulk_str(["key_none", "key_none", "key_none", "key_none"])) );
}


fn test_protomem() {

    let mut rm = rustymem::new_with_protocol("127.0.0.1", P_ASCII);

    let pm = rm.get_connection(0);

    println( fmt!("version: %?", pm.p_version()) );
    test_p_set(pm);
    test_p_add(pm);
    test_p_append(pm);
    test_p_get(pm);
    test_p_get2(pm);
    test_p_get3(pm);
    test_p_gets2(pm);
    test_p_stats(pm);
}

fn test_p_set(rm : &mut ~ProtoConnection) {
    println( fmt!("p_set: %?", rm.p_set("abc", bytes!("val123"), 0, 0, 0, false)) );
    println( fmt!("p_set: %?", rm.p_set("abc", bytes!("val124"), 0, 0, 0, true)) );

    let data = "xyz".to_owned();
    let data2 = data.into_bytes();
    println( fmt!("p_set: %?", rm.p_set("set1", data2, 0, 0, 60, false)) );
}

fn test_p_add(rm : &mut ~ProtoConnection) {
    rm.p_add(&"add1", bytes!("xyz"), 0, 0, 60, false);
}

fn test_p_append(rm : &mut ~ProtoConnection) {
    rm.p_append(&"add1", bytes!("-suffix1"), false);
}

fn test_p_get(rm : &mut ~ProtoConnection) {
    println( fmt!("get set1 : %?", rm.p_get(["set1"])) );
}

fn test_p_get2(rm : &mut ~ProtoConnection) {
    let mds = rm.p_get(["add1"]);
    println( fmt!("get add1 : %?", mds) );
    let list = mds;
    for md in list.iter() {
        println("md: " + md.key + "=" + md.to_str());
    }
    // mds.unwrap().each(|md| {
    //         println(md.key);
    //     });
}

fn test_p_get3(rm : &mut ~ProtoConnection) {
    println( fmt!("get abc set1 : %?", rm.p_get(["abc", "set1"])) );
}

fn test_p_gets2(rm : &mut ~ProtoConnection) {
    println( fmt!("gets abc set1 : %?", rm.p_gets(["abc", "set1"])) );
}

fn test_p_stats(rm : &mut ~ProtoConnection) {
    println( fmt!("stats : %?", rm.p_stats()) );
}


fn test_rm(rm: &mut RustyMem) {

    // Put some data in.

    println( fmt!("set_bytes key1 key1value: %?", rm.set_bytes("key1", 0, bytes!("key1value"))) );

    println( fmt!("set_str key2 key2value: %?", rm.set_str("key2", 0, "key2value")) );

    println( fmt!("set_json key3 300: %?", rm.set_json("key3", 0, ~300)) );
    println( fmt!("set_json key4 400u64: %?", rm.set_json("key4", 0, ~400u64)) );
    println( fmt!("set_json key5 500.5f: %?", rm.set_json("key5", 0, ~500.5f)) );

    println( fmt!("set_to_str tostr1 : %?", rm.set_to_str("tostr1", 0, &10)) );
    println( fmt!("set_to_str tostr2 : %?", rm.set_to_str("tostr2", 0, &20.0)) );
    println( fmt!("set_to_str tostr3 : %?", rm.set_to_str("tostr3", 0, &true)) );
    println( fmt!("set_to_str tostr4 : %?", rm.set_to_str("tostr4", 0, &false)) );

    let bytes1 = ~vec::from_elem(10, 65u8);
    println( fmt!("set_json key6 aa-byte: %?", rm.set_json("key6", 0, bytes1)) );
    println( fmt!("set_json key7 aa-byte: %?", rm.set_json("key7", 0, &10)) );
    println( fmt!("set_json key8 aa-byte: %?", rm.set_json("key8", 0, &20)) );
    println( fmt!("set_json key9 aa-byte: %?", rm.set_json("key9", 0, &true)) );

    // TODO: string doesn't work with ToJson somehow
    // let str1 = ~"key10value";
    // println( fmt!("set_json key10 key10value: %?", rm.set_json("key10", 0, str1) ) );


    // Get back the data.

    println( fmt!("get_bytes key1: %?", rm.get_bytes("key1")) );
    println( fmt!("get_bytes key1: %?", str::from_utf8(rm.get_bytes("key1").unwrap())) );
    println( fmt!("get_bytes key_none: %?", rm.get_bytes("key_none")) );
    
    println( fmt!("get_str key2: %?", rm.get_str("key2")) );
    println( fmt!("get_str key_none: %?", rm.get_str("key_none")) );

    println( fmt!("get_str key6: %?", rm.get_str("key6")) );
    println( fmt!("get_json key6: %?", rm.get_json("key6")) );
    println( fmt!("get_json key7: %?", rm.get_json("key7")) );
    println( fmt!("get_json key8: %?", rm.get_json("key8")) );
    println( fmt!("get_json key9: %?", rm.get_json("key9")) );

    println( fmt!("get_json key3: %?", rm.get_json("key3")) );
    println( fmt!("get_json key4: %?", rm.get_json("key4")) );
    println( fmt!("get_json key5: %?", rm.get_json("key5")) );

    let data : Option<MemData> = rm.get_data("key1");

    println( fmt!("get_data key1 : *** %?", data) );
    println( fmt!("get_data key1 : %?", rm.get_data("key1")) );
    println( fmt!("get_data key1 : %?", rm.get_data("key1").unwrap().as_bytes()) );
    println( fmt!("get_data key1 : %?", rm.get_data("key1").unwrap().as_str()) );
    println( fmt!("get_data tostr2 : %?", rm.get_data("tostr2").unwrap().as_from_str::<int>()) );
    println( fmt!("get_data tostr2 : %?", rm.get_data("tostr2").unwrap().as_from_str::<bool>()) );
    println( fmt!("get_data tostr2 : %?", rm.get_data("tostr2").unwrap().as_from_str_with(true)) );

    let md = rm.get_data("key1").unwrap();
    println( fmt!("get_data %? : %?", md.key, md) );
    let ptr = md.as_data_ptr();
    println( fmt!("get_data_ptr %? : %?", md.key, ptr) );
    println( fmt!("get_data_ptr key1 : %?", str::from_utf8(*ptr)) );

    println( fmt!("get_from_str tostr1 : %?", rm.get_from_str::<int>("tostr1")) );
    println( fmt!("get_from_str tostr2 : %?", rm.get_from_str::<float>("tostr2")) );
    println( fmt!("get_from_str tostr3 : %?", rm.get_from_str::<bool>("tostr3")) );
    println( fmt!("get_from_str tostr4 : %?", rm.get_from_str::<bool>("tostr4")) );

    println( fmt!("get_bulk_bytes key2 key5: %?", rm.get_bulk_bytes(["key2", "key5"])) );
    println( fmt!("get_bulk_str key2 key5: %?", rm.get_bulk_str(["key2", "key5"])) );
    println( fmt!("get_bulk_json key2 key5: %?", rm.get_bulk_json(["key2", "key5"])) );

    println( fmt!("get_bulk_from_str tostr3 tostr4: %?", rm.get_bulk_from_str::<bool>(["tostr3", "tostr4"])) );

    // CAS data
    let md = rm.get_data("key1").unwrap();
    println( fmt!("get_data for cas %? : %?, %? %?", md.key, md.cas, md, md.as_str()) );
    println( fmt!("cas_bytes key1 key1valueNotSet: %?", rm.cas_bytes("key1", 0, 0, bytes!("key1valueNotSet"))) );
    let md = rm.get_data("key1").unwrap();
    println( fmt!("get_data for cas %? : %?, %?", md.key, md.cas, md.as_str()) );

    println( fmt!("cas_bytes key1 key1valueSet: %?", rm.cas_bytes("key1", md.cas, 0, bytes!("key1valueSet"))) );
    let md = rm.get_data("key1").unwrap();
    println( fmt!("get_data for cas %? : %?, %?", md.key, md.cas, md.as_str()) );


    // Data functions
    println( fmt!("touch key1 : %?", rm.touch("key1", 10)) );
    let md = rm.get_data("key1").unwrap();
    println( fmt!("get_data %? : %?, %? %?", md.key, md.cas, md, md.as_str()) );

    println( fmt!("touch key1-nonexisted : %?", rm.touch("key1-nonexisted", 10)) );

    println( fmt!("delete key1 : %?", rm.delete("key1")) );
    println( fmt!("get_str key1: %?", rm.get_str("key1")) );

    println( fmt!("set_to_str num1 : %?", rm.set_to_str("num1", 0, &10)) );
    println( fmt!("incr num1 : %?", rm.incr("num1", 3)) );
    println( fmt!("get_from_str num1 : %?", rm.get_from_str::<u64>("num1")) );
    println( fmt!("incr num1 : %?", rm.incr("num1", 2)) );
    println( fmt!("get_from_str num1 : %?", rm.get_from_str::<u64>("num1")) );

}

fn main()  {

    debug!("main() enter");

    let mut rm = test_new_conn();
    test_rm(rm);

    test_protomem();

    test_binary_conn();

    test_cluster();

    let mut map = HashMap::<&str,~str>::new();
    map.insert(&"abc", ~"xyz");

    debug!("main() exit");

}



#[bench]
fn bench_baseline(b: &mut extra::test::BenchHarness) {
    do b.iter {
    }
    b.bytes = "key1value".len() as u64;
}

#[bench]
fn bench_versions_a(b: &mut extra::test::BenchHarness) {
    let mut rm = rustymem::new_with_protocol("127.0.0.1:11211", P_ASCII);
    rm.set_bytes("key1", 2*60*60, bytes!("key1value"));
    do b.iter {
        rm.versions();
    }
}

#[bench]
fn bench_versions_b(b: &mut extra::test::BenchHarness) {
    let mut rm = rustymem::new_with_protocol("127.0.0.1:11211", P_BINARY);
    rm.set_bytes("key1", 2*60*60, bytes!("key1value"));
    do b.iter {
        rm.versions();
    }
}

#[bench]
fn bench_get_1_key_a(b: &mut extra::test::BenchHarness) {
    let mut rm = rustymem::new_with_protocol("127.0.0.1:11211", P_ASCII);
    rm.set_bytes("key1", 2*60*60, bytes!("key1value"));
    do b.iter {
        rm.get_bytes("key1");
    }
}

#[bench]
fn bench_get_1_key_b(b: &mut extra::test::BenchHarness) {
    let mut rm = rustymem::new_with_protocol("127.0.0.1:11211", P_BINARY);
    rm.set_bytes("key1", 2*60*60, bytes!("key1value"));
    do b.iter {
        rm.get_bytes("key1");
    }
}

#[bench]
fn bench_get_1_none_key_a(b: &mut extra::test::BenchHarness) {
    let mut rm = rustymem::new_with_protocol("127.0.0.1:11211", P_ASCII);
    rm.set_bytes("key1", 2*60*60, bytes!("key1value"));
    do b.iter {
        rm.get_bytes("key_none");
    }
}

#[bench]
fn bench_get_1_none_key_b(b: &mut extra::test::BenchHarness) {
    let mut rm = rustymem::new_with_protocol("127.0.0.1:11211", P_BINARY);
    rm.set_bytes("key1", 2*60*60, bytes!("key1value"));
    do b.iter {
        rm.get_bytes("key_none");
    }
}

#[bench]
fn bench_get_keys_a(b: &mut extra::test::BenchHarness) {
    let mut rm = rustymem::new_with_protocol("127.0.0.1:11211", P_ASCII);
    rm.set_bytes("key1", 60, bytes!("key1value"));
    rm.set_bytes("key2", 60, bytes!("key2value"));
    rm.set_bytes("key3", 60, bytes!("key3value"));
    rm.set_bytes("key4", 60, bytes!("key4value"));
    rm.set_bytes("key5", 60, bytes!("key5value"));

    do b.iter {
        rm.get_bulk_str(["key1", "key2", "key3", "key4", "key5"]);
    }
}

#[bench]
fn bench_get_keys_b(b: &mut extra::test::BenchHarness) {
    let mut rm = rustymem::new_with_protocol("127.0.0.1:11211", P_BINARY);
    rm.set_bytes("key1", 60, bytes!("key1value"));
    rm.set_bytes("key2", 60, bytes!("key2value"));
    rm.set_bytes("key3", 60, bytes!("key3value"));
    rm.set_bytes("key4", 60, bytes!("key4value"));
    rm.set_bytes("key5", 60, bytes!("key5value"));

    do b.iter {
        rm.get_bulk_str(["key1", "key2", "key3", "key4", "key5"]);
    }
}

#[bench]
fn bench_set_1_key_a(b: &mut extra::test::BenchHarness) {
    let mut rm = rustymem::new_with_protocol("127.0.0.1:11211", P_ASCII);
    do b.iter {
        rm.set_bytes("key1", 2*60*60, bytes!("key1value"));
    }
}

#[bench]
fn bench_set_1_key_b(b: &mut extra::test::BenchHarness) {
    let mut rm = rustymem::new_with_protocol("127.0.0.1:11211", P_BINARY);
    do b.iter {
        rm.set_bytes("key1", 2*60*60, bytes!("key1value"));
    }
}

#[bench]
fn bench_set_1_1K_a(b: &mut extra::test::BenchHarness) {
    let mut rm = rustymem::new_with_protocol("127.0.0.1:11211", P_ASCII);
    let buf = vec::from_elem(1024, 0xABu8);
    do b.iter {
        rm.set_bytes("key1", 2*60*60, buf);
    }
}

#[bench]
fn bench_set_1_1K_b(b: &mut extra::test::BenchHarness) {
    let mut rm = rustymem::new_with_protocol("127.0.0.1:11211", P_BINARY);
    let buf = vec::from_elem(1024, 0xABu8);
    do b.iter {
        rm.set_bytes("key1", 2*60*60, buf);
    }
}

#[bench]
fn bench_set_1_10K_a(b: &mut extra::test::BenchHarness) {
    let mut rm = rustymem::new_with_protocol("127.0.0.1:11211", P_ASCII);
    let buf = vec::from_elem(1024*10, 0xABu8);
    do b.iter {
        rm.set_bytes("key1", 2*60*60, buf);
    }
}

#[bench]
fn bench_set_1_10K_b(b: &mut extra::test::BenchHarness) {
    let mut rm = rustymem::new_with_protocol("127.0.0.1:11211", P_BINARY);
    let buf = vec::from_elem(1024*10, 0xABu8);
    do b.iter {
        rm.set_bytes("key1", 2*60*60, buf);
    }
}

#[bench]
fn bench_set_1_100K_a(b: &mut extra::test::BenchHarness) {
    let mut rm = rustymem::new_with_protocol("127.0.0.1:11211", P_ASCII);
    let buf = vec::from_elem(1024*100, 0xABu8);
    do b.iter {
        rm.set_bytes("key1", 2*60*60, buf);
    }
}

#[bench]
fn bench_set_1_100K_b(b: &mut extra::test::BenchHarness) {
    let mut rm = rustymem::new_with_protocol("127.0.0.1:11211", P_BINARY);
    let buf = vec::from_elem(1024*100, 0xABu8);
    do b.iter {
        rm.set_bytes("key1", 2*60*60, buf);
    }
}

