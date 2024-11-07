import { Config } from "tailwindcss";

export default {
  content: [
    "./index.html",
    "./src-www/**/*.{ts,tsx}",
  ],
  theme: {
    extend: {},
  },
  plugins: [],
} satisfies Config;
