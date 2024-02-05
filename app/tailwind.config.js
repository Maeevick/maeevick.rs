module.exports = {
    mode: "jit",
    content: {
        files: ["src/**/*.rs", "index.html"],
    },
    darkMode: "media",
    theme: {
        extend: {
            colors: {
                'maeevick-orange': '#ff6100',
            },
        },
    },
    variants: {
        extend: {},
    },
    plugins: [],
};