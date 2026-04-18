use std::sync::{Mutex, MutexGuard, OnceLock};

#[cfg(not(target_arch = "wasm32"))]
use std::collections::HashMap;

#[cfg(not(target_arch = "wasm32"))]
const STORAGE_FILE: &str = "storage.data";

pub struct Storage;

impl Storage {
    pub fn get(key: &str) -> Option<String> {
        lock().get(key)
    }
    pub fn set(key: &str, value: &str) {
        lock().set(key, value)
    }
    pub fn remove(key: &str) {
        lock().remove(key)
    }
    pub fn clear() {
        lock().clear()
    }
    pub fn len() -> usize {
        lock().len()
    }
    pub fn is_empty() -> bool {
        Self::len() == 0
    }
    pub fn key(index: usize) -> Option<String> {
        lock().key(index)
    }
}

fn lock() -> MutexGuard<'static, Inner> {
    static INNER: OnceLock<Mutex<Inner>> = OnceLock::new();
    INNER
        .get_or_init(|| Mutex::new(Inner::new()))
        .lock()
        .unwrap()
}

struct Inner {
    #[cfg(not(target_arch = "wasm32"))]
    map: HashMap<String, String>,
}

impl Inner {
    fn new() -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            Self {}
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let mut s = Self {
                map: HashMap::new(),
            };
            s.load();
            s
        }
    }

    fn get(&self, key: &str) -> Option<String> {
        #[cfg(target_arch = "wasm32")]
        {
            wasm::get(key)
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.map.get(key).cloned()
        }
    }

    fn set(&mut self, key: &str, value: &str) {
        #[cfg(target_arch = "wasm32")]
        {
            wasm::set(key, value);
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.map.insert(key.to_string(), value.to_string());
            self.save();
        }
    }

    fn remove(&mut self, key: &str) {
        #[cfg(target_arch = "wasm32")]
        {
            wasm::remove(key);
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.map.remove(key);
            self.save();
        }
    }

    fn clear(&mut self) {
        #[cfg(target_arch = "wasm32")]
        {
            wasm::clear();
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.map.clear();
            self.save();
        }
    }

    fn len(&self) -> usize {
        #[cfg(target_arch = "wasm32")]
        {
            wasm::len()
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.map.len()
        }
    }

    fn key(&self, index: usize) -> Option<String> {
        #[cfg(target_arch = "wasm32")]
        {
            wasm::key(index)
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.map.keys().nth(index).cloned()
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn load(&mut self) {
        let Ok(data) = std::fs::read(STORAGE_FILE) else {
            return;
        };
        let mut i = 0;
        while i < data.len() {
            let Some((k, ni)) = read_field(&data, i) else {
                break;
            };
            let Some((v, nj)) = read_field(&data, ni) else {
                break;
            };
            self.map.insert(k, v);
            i = nj;
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn save(&self) {
        let mut out = Vec::new();
        for (k, v) in &self.map {
            write_field(&mut out, k);
            write_field(&mut out, v);
        }
        let _ = std::fs::write(STORAGE_FILE, out);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn read_field(data: &[u8], i: usize) -> Option<(String, usize)> {
    let len_end = i.checked_add(4)?;
    let len_bytes: [u8; 4] = data.get(i..len_end)?.try_into().ok()?;
    let len = u32::from_le_bytes(len_bytes) as usize;
    let end = len_end.checked_add(len)?;
    let s = std::str::from_utf8(data.get(len_end..end)?)
        .ok()?
        .to_string();
    Some((s, end))
}

#[cfg(not(target_arch = "wasm32"))]
fn write_field(out: &mut Vec<u8>, s: &str) {
    out.extend_from_slice(&(s.len() as u32).to_le_bytes());
    out.extend_from_slice(s.as_bytes());
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    unsafe extern "C" {
        fn storage_get_len(key_ptr: *const u8, key_len: u32) -> i32;
        fn storage_get_copy(out_ptr: *mut u8);
        fn storage_set(key_ptr: *const u8, key_len: u32, val_ptr: *const u8, val_len: u32);
        fn storage_remove(key_ptr: *const u8, key_len: u32);
        fn storage_clear();
        fn storage_len() -> u32;
        fn storage_key_len(index: u32) -> i32;
        fn storage_key_copy(out_ptr: *mut u8);
    }

    pub fn get(key: &str) -> Option<String> {
        unsafe {
            let len = storage_get_len(key.as_ptr(), key.len() as u32);
            if len < 0 {
                return None;
            }
            let mut buf = vec![0u8; len as usize];
            storage_get_copy(buf.as_mut_ptr());
            String::from_utf8(buf).ok()
        }
    }

    pub fn set(key: &str, value: &str) {
        unsafe {
            storage_set(
                key.as_ptr(),
                key.len() as u32,
                value.as_ptr(),
                value.len() as u32,
            );
        }
    }

    pub fn remove(key: &str) {
        unsafe {
            storage_remove(key.as_ptr(), key.len() as u32);
        }
    }

    pub fn clear() {
        unsafe {
            storage_clear();
        }
    }

    pub fn len() -> usize {
        unsafe { storage_len() as usize }
    }

    pub fn key(index: usize) -> Option<String> {
        unsafe {
            let len = storage_key_len(index as u32);
            if len < 0 {
                return None;
            }
            let mut buf = vec![0u8; len as usize];
            storage_key_copy(buf.as_mut_ptr());
            String::from_utf8(buf).ok()
        }
    }
}
