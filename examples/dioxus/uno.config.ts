import {
  defineConfig,
  presetTypography,
  transformerVariantGroup,
  presetIcons,
  presetWebFonts,
  presetUno,
} from "unocss";

export default defineConfig({
  safelist: [],

  presets: [
    presetUno(),
    presetTypography(),
    presetIcons({ prefix: "icon-", scale: 1.2, warn: true }),
    presetWebFonts({
      fonts: { sans: "DM Sans", serif: "Bree Serif" },
    }),
  ],

  transformers: [transformerVariantGroup()],

  content: { pipeline: { include: ["**/*.rs"] } },

  cli: {
    entry: {
      patterns: ["src/**/*.rs"],
      outFile: "public/uno.css",
    },
  },
});
