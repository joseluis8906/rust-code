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
        active: '#464646',
        primary: {
          500: '#3584e4',
          550: '#3f8fee',
          600: '#4999f8',
        },
      },
    },
  },
  plugins: [],
}
