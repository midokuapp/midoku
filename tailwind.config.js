/** @type {import('tailwindcss').Config} */
module.exports = {
    mode: "jit",
    content: ["*.html", "./src-ui/src/**/*.rs"],
    theme: {
        extend: {},
    },
    plugins: [require("daisyui")],
};
