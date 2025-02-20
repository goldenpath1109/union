import icon from "astro-icon"
import { loadEnv } from "vite"
import svelte from "@astrojs/svelte"
import sitemap from "@astrojs/sitemap"
import netlify from "@astrojs/netlify"
import tailwind from "@astrojs/tailwind"
import { defineConfig } from "astro/config"
import { markdownConfiguration } from "./markdown.config.ts"

const SITE_URL = "https://union.build"

const { PORT = 4321, ENABLE_DEV_TOOLBAR = "false" } = loadEnv(
  process.env.NODE_ENV,
  process.cwd(),
  ""
)

export default defineConfig({
  site: SITE_URL,
  output: "hybrid",
  experimental: {
    serverIslands: true,
    clientPrerender: true,
    directRenderScript: true
  },
  trailingSlash: "ignore",
  adapter: netlify({
    imageCDN: false,
    edgeMiddleware: false
  }),
  image: {
    domains: [
      "cdn.contentful.com",
      "images.ctfassets.net",
      "raw.githubusercontent.com",
      "avatars.githubusercontent.com"
    ]
  },
  markdown: markdownConfiguration,
  server: _ => ({ port: Number(PORT) }),
  devToolbar: { enabled: ENABLE_DEV_TOOLBAR === "true" },
  prefetch: { prefetchAll: true, defaultStrategy: "viewport" },
  redirects: {
    "/feed": "/rss.xml",
    "/logo": "/union-logo.zip",
    "/docs": "https://docs.union.build"
  },
  vite: {
    assetsInclude: ["**/*.splinecode"],
    optimizeDeps: {
      exclude: ["echarts"]
    },
    define: {
      global: {}
    }
  },
  integrations: [
    icon(),
    tailwind({
      applyBaseStyles: false,
      configFile: "tailwind.config.ts"
    }),
    svelte(),
    sitemap()
  ]
})
