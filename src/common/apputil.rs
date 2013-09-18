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



//use std::Path;


pub fn get_program(args: &~[~str]) -> ~str {
    let path : Path = GenericPath::from_str((*args)[0]);
    let name = path.filestem();
    debug!( fmt!("name = %?", name) );

    match name {
        Some(name) => { name.to_owned() },
        None => ~""
    }
}

