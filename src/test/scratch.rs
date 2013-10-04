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



trait Trait1 {
    fn func1(&self);
}

struct Sub1 {
    subvalue1: uint
}

struct Sub2 {
    subvalue2: uint
} 

impl Trait1 for Sub1 {
    fn func1(&self) {
        println(fmt!("func1() of Sub1 %? ", self.subvalue1));
    }
}

impl Trait1 for Sub2 {
    fn func1(&self) {
        println(fmt!("func1() of Sub2 %? ", self.subvalue2));
    }
}


struct Foo<T> {
    subobj:    T
}

impl<T: Trait1> Foo<T> {
    fn new(sub: T) -> Foo<T> {
        return Foo::<T> {
            subobj: sub
        }
    }
}


struct Bar<T> {
    subobj:    T
}

impl<T> Bar<T> {
    fn new() -> Bar<int> {
        Bar {
            subobj: 1
        };
    }
}



// Test bed for trying things out.
fn main()  {

    debug!("scratch main() enter");

    let str1 = ~"this is a test ";
    let str1trim = str1.trim();
    println(fmt!( "%?", str1 ));
    println(fmt!( "%?", str1trim));
    let str1trim2 = str1.trim().to_owned();
    println(fmt!( "%?", str1trim2 ));

    let strs1 = str1.split_iter(' ').to_owned_vec();
    println(fmt!( "%?", strs1 ));

    let strs2 = ~[~"abc", ~"xyz"];
    println(fmt!( "%?", strs2 ));

    debug!("scratch main() exit");

    Sub1 { subvalue1: 1 }.func1();
    Sub2 { subvalue2: 2 }.func1();

    println(fmt!( "%?", Foo::new( Sub1 { subvalue1: 3 } ) ));
    println(fmt!( "%?", Foo::new( Sub2 { subvalue2: 4 } ) ));

    println(fmt!( "%?", Bar::new() ));

    // let bar1 : Bar<Sub1> = Bar::<Sub1> {
    //     subobj: Sub1 {subvalue1: 1}
    // }

}

