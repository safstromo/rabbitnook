/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    relative: true,
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {
      fontFamily: {
        sans: ["JetBrains Mono", "monospace"],
      },
      colors: {
        base: "#1E1E2E",
        sky: "#91d7e3",
        blue: "#8aadf4",
        maroon: "#ee99a0",
        peach: "#f5a97f",
        green: "#a6da95",
        teal: "#8bd5ca",
        surface: "#363a4f",
        pink: "#f5bde6",
      },
    },
  },
  plugins: [],
};
