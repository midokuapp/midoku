import { Config } from "tailwindcss";
import daisyui from "daisyui";

export default {
  content: [
    "./index.html",
    "./src/**/*.{ts,tsx}",
  ],
  theme: {
    extend: {},
  },
  plugins: [daisyui],
} satisfies Config;
