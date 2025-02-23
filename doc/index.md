---
# https://vitepress.dev/reference/default-theme-home-page
layout: home

hero:
  name: Activation-Manager
  text: Post-modern impure configuration managemenent
  tagline: Documentation
  # actions:
  #   - theme: brand
  #     text: Options
  #     link: /options

# features:
#   - title: Feature A
#     details: Lorem ipsum dolor sit amet, consectetur adipiscing elit
#   - title: Feature B
#     details: Lorem ipsum dolor sit amet, consectetur adipiscing elit
#   - title: Feature C
#     details: Lorem ipsum dolor sit amet, consectetur adipiscing elit
---


<h2>Options</h2>

<script setup>
import { data } from "./nixos.data.ts"
</script>

<div v-for="elem in data" :key="elem.loc">
  <h3>{{ elem.loc.join(".") }}</h3>
  <p>{{elem.description}}</p>
</div>
