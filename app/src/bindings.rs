#[cfg(target_arch = "wasm32")]
unsafe extern "C" {
    fn js_reload();
}

#[cfg(target_arch = "wasm32")]
pub fn reload() {
    unsafe { js_reload() };
}

#[cfg(not(target_arch = "wasm32"))]
pub fn reload() {
    std::process::exit(0);
}
