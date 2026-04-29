function initToolbar() {
  document.querySelector("#btn-back")?.addEventListener("click", () => {
    window.history.back();
  });
  document.querySelector("#btn-forward")?.addEventListener("click", () => {
    window.history.forward();
  });
  document.querySelector("#btn-share")?.addEventListener("click", () => {
    console.log("Share");
  });
  document.querySelector("#btn-sidebar")?.addEventListener("click", () => {
    document.body.classList.toggle("sidebar-open");
  });
}

window.addEventListener("DOMContentLoaded", () => {
  initToolbar();
});
