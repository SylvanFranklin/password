/** @type {import('tailwindcss').Config}*/
const config = {
    content: ["./src/**/*.{html,js,svelte,ts}"],

    // all gradients optionts have to be safelisted
    safelist: [

    ],

    theme: {
        extend: {},
    },

    plugins: [],
};

module.exports = config;
