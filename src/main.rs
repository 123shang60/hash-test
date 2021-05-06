use dashmap::DashMap;
use fnv::FnvHashMap;
use highwayhash::HighwayHasher;
use std::hash::BuildHasherDefault;
use std::hash::SipHasher;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::SystemTime,
};

type highwayBuilderHasher = BuildHasherDefault<HighwayHasher>;
type SipBuilderHasher = BuildHasherDefault<SipHasher>;

fn main() {
    let mut map = HashMap::new();
    let sy_time = SystemTime::now();
    for i in 0..1000000 {
        map.insert(i, i);
    }
    println!(
        "{},{}",
        "原生hashmap消耗用时",
        SystemTime::now()
            .duration_since(sy_time)
            .unwrap()
            .as_millis()
    );
    let mut map = FnvHashMap::default();
    let sy_time = SystemTime::now();
    for i in 0..1000000 {
        map.insert(i, i);
    }
    println!(
        "{},{}",
        "fnvHashMap消耗用时",
        SystemTime::now()
            .duration_since(sy_time)
            .unwrap()
            .as_millis()
    );

    let mut map = HashMap::with_hasher(SipBuilderHasher::default());
    let sy_time = SystemTime::now();
    for i in 0..1000000 {
        map.insert(i, i);
    }
    println!(
        "{},{}",
        "SipHashmap消耗用时",
        SystemTime::now()
            .duration_since(sy_time)
            .unwrap()
            .as_millis()
    );

    let mut map = HashMap::with_hasher(highwayBuilderHasher::new());
    let sy_time = SystemTime::now();
    for i in 0..1000000 {
        map.insert(i, i);
    }
    println!(
        "{},{}",
        "highwayHashmap消耗用时",
        SystemTime::now()
            .duration_since(sy_time)
            .unwrap()
            .as_millis()
    );

    let map = Arc::new(RwLock::new(HashMap::new()));
    let sy_time = SystemTime::now();
    for i in 0..10000 {
        let wr = map.clone();
        let handle = std::thread::spawn(move || {
            if let Ok(mut wr) = wr.write() {
                wr.insert(i, i);
            }
        });
        handle.join().unwrap();
    }
    println!(
        "{},{}",
        "Arc<RwLock<HashMap>>消耗用时",
        SystemTime::now()
            .duration_since(sy_time)
            .unwrap()
            .as_millis()
    );
    let map = Arc::new(DashMap::new());
    let sy_time = SystemTime::now();
    for i in 0..10000 {
        let map = map.clone();
        let handle = std::thread::spawn(move || {
            map.insert(i, i);
        });
        handle.join().unwrap();
    }
    println!(
        "{},{}",
        "dashmap消耗用时",
        SystemTime::now()
            .duration_since(sy_time)
            .unwrap()
            .as_millis()
    );
}
