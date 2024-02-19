const CACHE_NAME = "maeevick-cache";

const cachedFiles = [
    "/",
    "/index.html",
    "/error.html",
    "/styles.css",
    "/manifest.json",
    "/robots.txt",
    "/favicon.ico",
    "/images/maeevick-banner.webp",
    "/images/microsaas-maker.webp",
    "/images/icons/icon-192x192.png",
    "/images/icons/icon-384x384.png",
    "/images/icons/icon-512x512.png",
    "/images/icons/icon-mask-512x512.png",
    "/images/icons/icon-1024x1024.png",
];

self.addEventListener('install', function (event) {
    event.waitUntil(
        caches.open(CACHE_NAME).then(function (cache) {
            return cache.addAll(cachedFiles);
        })
    );
});

self.addEventListener('fetch', function (event) {
    event.respondWith(
        caches.match(event.request).then(function (response) {
            return response || fetch(event.request);
        }).catch(function (error) {
            return caches.match(new Request('/error.html'));
        })
    );
});