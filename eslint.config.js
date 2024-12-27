import pluginAstro from "eslint-plugin-astro";

export default [
  ...pluginAstro.configs["flat/recommended"],
  ...pluginAstro.configs["flat/jsx-a11y-recommended"],
];
