# Hello

Blah

<script setup>
import { data } from "./nixos.data.ts"

const rawHTML = "<pre>hello</pre>"
</script>

<div v-for="elem in data" :key="elem.loc">
  <h3>{{ elem.loc.join(".") }}</h3>
  <span v-html="rawHTML"></span>
  <span v-html="elem.description"></span>
</div>
