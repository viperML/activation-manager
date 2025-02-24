---
# https://vitepress.dev/reference/default-theme-home-page
layout: home

hero:
  name: Activation-Manager
  text: Post-modern configuration managemenent
  # tagline: Documentation
  actions:
    - theme: brand
      text: GitHub
      link: https://github.com/viperML/activation-manager

# features:
#   - title: Feature A
#     details: Lorem ipsum dolor sit amet, consectetur adipiscing elit
#   - title: Feature B
#     details: Lorem ipsum dolor sit amet, consectetur adipiscing elit
#   - title: Feature C
#     details: Lorem ipsum dolor sit amet, consectetur adipiscing elit
---


<h2>Module options</h2>

<script setup>
import { data } from "./nixos.data.ts"
</script>

<div v-for="elem in data" :key="elem.loc">
  <h3>{{ elem.loc.join(".") }}</h3>
  <div v-html="elem.description"></div>
  <span>Type:</span>
  <pre>{{elem.type}}</pre>
</div>
