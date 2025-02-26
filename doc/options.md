---
title: Module
---

## Module options


<script setup>
import { data } from "./nixos.data.ts"

// const config = globalThis.VITEPRESS_CONFIG as SiteConfig
// const md = await createMarkdownRenderer(config.srcDir, config.markdown, config.site.base, config.logger)

// const t = "<h2>Test</h2>"
</script>

<div v-for="elem in data" :key="elem.loc">

  ### {{elem.loc.join(".")}}

  <div v-html="elem.description"></div>
  <span>Type:</span>
  <pre>{{elem.type}}</pre>
</div>
