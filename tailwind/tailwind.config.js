/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["../static/**/*.{html,js}"],
  theme: {
    extend: {
      colors: {
        'maeevick-orange': '#ff6100',
      },
      boxShadow: {
        'maeevick-cta': '10px 10px rgba(255, 97, 0, 1)',
      },
    },
  },
  plugins: [],
}

