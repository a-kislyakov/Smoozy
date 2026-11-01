// One no-op typed binding, called through whichever bridge the shell provides.
// No network request is made and no updater is referenced.
function ping(text) {
  if (window.go && window.go.main && window.go.main.App && window.go.main.App.Ping) {
    return window.go.main.App.Ping(text);
  }
  if (window.__TAURI__ && window.__TAURI__.core && window.__TAURI__.core.invoke) {
    return window.__TAURI__.core.invoke("ping", { text: text });
  }
  return Promise.resolve("");
}

document.addEventListener("DOMContentLoaded", function () {
  var echo = document.getElementById("echo");
  Promise.resolve(ping("ready")).then(function (answer) {
    echo.textContent = String(answer);
  });
  ["min", "max", "close"].forEach(function (id) {
    document.getElementById(id).addEventListener("click", function () {
      ping(id);
    });
  });
});
