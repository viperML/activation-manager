# Hello

Blah

<script setup>
import { data } from "./nixos.data.ts"
</script>

<div v-for="elem in data" :key="elem.loc">
  <h3>{{ elem.loc.join(".") }}</h3>
  <p>{{elem.description}}</p>
</div>
