// Copyright 2021 Palantir Technologies, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use conjure_object::{DoubleKey, Uuid};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;

fn serialize<T>(value: &T) -> Vec<u8>
where
    T: Serialize,
{
    crate::smile::to_vec(value).unwrap()
}

fn deserialize_client<T>(smile: &[u8]) -> T
where
    T: DeserializeOwned,
{
    crate::smile::client_from_slice(smile).unwrap()
}

fn deserialize_server<T>(smile: &[u8]) -> T
where
    T: DeserializeOwned,
{
    crate::smile::server_from_slice(smile).unwrap()
}

fn test_ser<T>(ty: &T, expected_smile: &[u8])
where
    T: Serialize,
{
    let actual_smile = serialize(ty);
    let expected_value =
        serde_smile::from_slice::<serde_smile::value::Value>(expected_smile).unwrap();
    let actual_value = serde_smile::from_slice::<serde_smile::value::Value>(&actual_smile).unwrap();

    assert_eq!(expected_value, actual_value);
}

fn test_de<T>(ty: &T, smile: &[u8])
where
    T: DeserializeOwned + PartialEq + Debug,
{
    let deserialized = deserialize_client(smile);
    assert_eq!(*ty, deserialized);

    let deserialized = deserialize_server(smile);
    assert_eq!(*ty, deserialized);
}

fn test_serde<T>(ty: &T, expected_smile: &[u8])
where
    T: Serialize + DeserializeOwned + PartialEq + Debug,
{
    test_ser(ty, expected_smile);
    test_de(ty, expected_smile);
}

#[test]
fn binary_serde() {
    test_serde(
        &ByteBuf::from(b"foobar".to_vec()),
        b":)\n\x05\xfd\x86foobar",
    );
}

#[test]
fn binary_keys() {
    test_serde(
        &BTreeMap::from([(ByteBuf::from(b"foobar".to_vec()), 0)]),
        b":)\n\x05\xfa\x87Zm9vYmFy\xc0\xfb",
    )
}

#[test]
fn boolean_keys() {
    test_serde(
        &BTreeMap::from([(false, 0), (true, 1)]),
        b":)\n\x05\xfa\x84false\xc0\x83true\xc2\xfb",
    );
}

#[test]
fn double_keys() {
    test_serde(
        &BTreeMap::from([
            (DoubleKey(f64::NEG_INFINITY), 0),
            (DoubleKey(-1.5), 1),
            (DoubleKey(1.5), 2),
            (DoubleKey(f64::INFINITY), 3),
            (DoubleKey(f64::NAN), 4),
        ]),
        b":)\n\x05\xfa\x88-Infinity\xc0\x83-1.5\xc2\x821.5\xc4\x87Infinity\xc6\x82NaN\xc8\xfb",
    )
}

#[test]
fn uuid_keys() {
    test_serde(
        &BTreeMap::from([(Uuid::nil(), 1)]),
        b":)\n\x05\xfa\xa300000000-0000-0000-0000-000000000000\xc2\xfb",
    );
}

#[test]
fn uuid_values() {
    test_serde(
        &Uuid::nil(),
        b":)\n\x05\xfd\x90\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
    )
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Foo {
    foo: i32,
}

#[test]
fn client_unknown_fields() {
    let deserialized = deserialize_client::<Foo>(b":)\n\x05\xfa\x82foo\xc2\x84bogusDhello\xfb");
    assert_eq!(Foo { foo: 1 }, deserialized);
}

#[test]
fn server_unknown_fields() {
    let smile = b":)\n\x05\xfa\x82foo\xc2\x84bogusDhello\xfb";

    let e = crate::smile::server_from_slice::<Foo>(smile).err().unwrap();

    assert!(e.to_string().contains("foo"));
    assert!(e.to_string().contains("bogus"));
}

#[derive(Deserialize, Debug, PartialEq)]
struct Collections {
    list: Vec<u32>,
    set: BTreeSet<u32>,
    map: BTreeMap<String, u32>,
}

#[test]
fn null_collections() {
    let smile = b":)\n\x05\xfa\x83list!\x82map!\x82set!\xfb";

    let expected = Collections {
        list: vec![],
        set: BTreeSet::new(),
        map: BTreeMap::new(),
    };

    let actual =
        Collections::deserialize(&mut crate::smile::ServerDeserializer::from_slice(smile)).unwrap();
    assert_eq!(expected, actual);

    let actual =
        Collections::deserialize(&mut crate::smile::ClientDeserializer::from_slice(smile)).unwrap();
    assert_eq!(expected, actual);
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct UuidField {
    uuid: Uuid,
}

#[test]
fn uuid_field() {
    let smile = b":)\n\x05\xfa\x83uuid\xfd\x90\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\xfb";

    let value = UuidField { uuid: Uuid::nil() };

    let actual = crate::smile::to_vec(&value).unwrap();
    assert_eq!(actual, smile);

    let actual = crate::smile::client_from_slice::<UuidField>(smile).unwrap();
    assert_eq!(actual, value);

    let actual = crate::smile::server_from_slice::<UuidField>(smile).unwrap();
    assert_eq!(actual, value);
}
