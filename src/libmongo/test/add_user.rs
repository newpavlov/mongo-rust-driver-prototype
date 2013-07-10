/* Copyright 2013 10gen Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use mongo::db::*;
use mongo::client::*;
use mongo::util::*;

use bson::encode::*;

#[test]
fn test_add_user() {
    let client = @Client::new();
    match client.connect(~"127.0.0.1", MONGO_DEFAULT_PORT) {
        Ok(_) => (),
        Err(e) => fail!("%s", MongoErr::to_str(e))
    }

    // drop users first
    let db = DB::new(~"rust_add_user", client);
    let coll = db.get_collection(~"system.users");
    coll.remove(None, None, None, None);
    match db.add_user(~"testuser", ~"testpassword", ~[]) {
        Ok(_) => (),
        Err(e) => fail!("%s", MongoErr::to_str(e))
    };

    let mut cursor = match coll.find(Some(SpecNotation(~"{ \"user\": \"testuser\" }")), None, None) {
        Ok(d) => d,
        Err(e) => fail!("%?", e)
    };

    let usr = cursor.next().unwrap();

    assert_eq!(usr.find(~"user").unwrap(), &UString(~"testuser"));
}
