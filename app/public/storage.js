(function () {
    let mem = null;
    let staged = null;
    let available = true;

    const decoder = new TextDecoder();
    const encoder = new TextEncoder();

    function getMem() {
        if (mem) return mem;
        if (typeof wasm_memory !== "undefined" && wasm_memory) {
            mem = wasm_memory;
        } else if (typeof wasm_exports !== "undefined" && wasm_exports && wasm_exports.memory) {
            mem = wasm_exports.memory;
        }
        return mem;
    }

    function readStr(ptr, len) {
        const m = getMem();
        if (!m) throw new Error("wasm memory not yet initialized");
        return decoder.decode(new Uint8Array(m.buffer, ptr, len));
    }

    function writeBytes(ptr, bytes) {
        const m = getMem();
        if (!m) throw new Error("wasm memory not yet initialized");
        new Uint8Array(m.buffer, ptr, bytes.length).set(bytes);
    }

    function stage(bytes) {
        staged = bytes;
        return bytes.length;
    }

    function emit(out) {
        if (staged) {
            writeBytes(out, staged);
            staged = null;
        }
    }

    miniquad_add_plugin({
        register_plugin: function (importObject) {
            importObject.env.storage_get_len = function (kp, kl) {
                staged = null;
                if (!available) return -1;
                try {
                    const v = localStorage.getItem(readStr(kp, kl));
                    if (v === null) return -1;
                    return stage(encoder.encode(v));
                } catch (e) {
                    console.warn("storage_get failed:", e);
                    return -1;
                }
            };
            importObject.env.storage_get_copy = function (out) {
                try {
                    emit(out);
                } catch (e) {
                    console.warn("storage_get_copy failed:", e);
                    staged = null;
                }
            };
            importObject.env.storage_set = function (kp, kl, vp, vl) {
                if (!available) return;
                try {
                    localStorage.setItem(readStr(kp, kl), readStr(vp, vl));
                } catch (e) {
                    // Most commonly: QuotaExceededError, or storage disabled.
                    console.warn("storage_set failed:", e);
                }
            };
            importObject.env.storage_remove = function (kp, kl) {
                if (!available) return;
                try {
                    localStorage.removeItem(readStr(kp, kl));
                } catch (e) {
                    console.warn("storage_remove failed:", e);
                }
            };
            importObject.env.storage_clear = function () {
                if (!available) return;
                try {
                    localStorage.clear();
                } catch (e) {
                    console.warn("storage_clear failed:", e);
                }
            };
            importObject.env.storage_len = function () {
                if (!available) return 0;
                try {
                    return localStorage.length;
                } catch (e) {
                    return 0;
                }
            };
            importObject.env.storage_key_len = function (i) {
                staged = null;
                if (!available) return -1;
                try {
                    const k = localStorage.key(i);
                    if (k === null) return -1;
                    return stage(encoder.encode(k));
                } catch (e) {
                    console.warn("storage_key failed:", e);
                    return -1;
                }
            };
            importObject.env.storage_key_copy = function (out) {
                try {
                    emit(out);
                } catch (e) {
                    console.warn("storage_key_copy failed:", e);
                    staged = null;
                }
            };
        },
        on_init: function (wasm_memory) {
            if (wasm_memory) mem = wasm_memory;
            try {
                localStorage.getItem("__probe");
            } catch (e) {
                available = false;
                console.warn("localStorage unavailable; storage plugin is inert:", e);
            }
        },
        name: "storage",
        version: "0.2"
    });
})();