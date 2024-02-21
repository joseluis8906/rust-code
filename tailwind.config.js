/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {
      colors: {
        background: '#242424',
        foreground: '#323232',
        hover: '#3c3c3c',
      },
    },
  },
  plugins: [],
}
