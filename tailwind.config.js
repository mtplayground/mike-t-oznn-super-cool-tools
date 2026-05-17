/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./crates/toolbox-shell/src/**/*.rs",
    "./crates/tools/**/*.rs",
  ],
  theme: {
    extend: {
      colors: {
        accent: {
          DEFAULT: "#22d3ee",
          deep: "#0f172a",
        },
      },
      boxShadow: {
        glow: "0 30px 120px rgba(34, 211, 238, 0.18)",
      },
    },
  },
  plugins: [],
};
