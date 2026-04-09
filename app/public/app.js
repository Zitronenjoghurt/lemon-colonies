(async function () {
    const loginScreen = document.getElementById("login-screen");
    const canvas = document.getElementById("glcanvas");
    const spinner = document.getElementById("loading-spinner");

    try {
        const res = await fetch("/api/me", {credentials: "same-origin"});
        spinner.style.display = "none";

        if (res.ok) {
            canvas.style.display = "block";
            const s = document.createElement("script");
            s.src = "public/mq_js_bundle.js";
            s.onload = () => load("lemon-colonies-app.wasm");
            document.body.appendChild(s);
        } else {
            loginScreen.style.display = "flex";
        }
    } catch {
        spinner.style.display = "none";
        loginScreen.style.display = "flex";
    }

    window.logout = function () {
        window.location.reload();
    };
})();