/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {
      colors: {
        osd: '#171717',
        background: '#242424',
        foreground: '#333333',
        hover: '#3c3c3c',
      },
    },
  },
  plugins: [],
}
