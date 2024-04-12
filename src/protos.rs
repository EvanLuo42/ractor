pub mod scenes {
    include!(concat!(env!("OUT_DIR"), "/scenes.rs"));
}

pub mod global {
    include!(concat!(env!("OUT_DIR"), "/global.rs"));
}
