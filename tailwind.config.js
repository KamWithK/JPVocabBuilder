module.exports = {
  mode: 'jit',
  purge: [
    './public/index.html',
    './src/svelte/**/*.svelte'
  ],
  darkMode: false, // or 'media' or 'class'
  theme: {
    extend: {},
  },
  variants: {
    extend: {},
  },
  plugins: [],
}
