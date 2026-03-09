if (document.readyState !== "loading") {
    initHeroBadge();
} else {
    document.addEventListener("DOMContentLoaded", initHeroBadge);
}

function initHeroBadge() {
    const badge = document.querySelector(".badge");
    if (!badge) {
        return;
    }

    let active = false;
    window.setInterval(() => {
        active = !active;
        badge.classList.toggle("badge-primary", active);
        badge.classList.toggle("badge-outline", !active);
    }, 1800);
}
