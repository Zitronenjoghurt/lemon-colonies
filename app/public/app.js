(async function () {
    const loginScreen = document.getElementById("login-screen");
    const canvas = document.getElementById("glcanvas");
    const spinner = document.getElementById("loading-spinner");

    try {
        const res = await fetch("/api/me", {credentials: "same-origin"});
        spinner.style.display = "none";

        if (res.ok) {
            canvas.style.display = "block";
            canvas.addEventListener("contextmenu", (e) => e.preventDefault());
            miniquad_add_plugin({
                name: "lemon_colonies_helpers",
                version: "1.0.0",
                register_plugin: function (importObject) {
                    importObject.env.js_reload = function () {
                        window.location.reload();
                    };
                }
            });
            load("lemon-colonies-app.wasm");
        } else {
            loginScreen.style.display = "flex";
        }
    } catch {
        spinner.style.display = "none";
        loginScreen.style.display = "flex";
    }
})();