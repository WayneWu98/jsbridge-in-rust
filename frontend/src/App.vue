<script setup lang="ts">
import { invoke, ActionType, download } from './bridge';
const send = () => {
  download({ url: 'https://static.wayne-wu.com/me-in-2022_2023-01-02-23:57:35.png?imageView2/2/w/1200/q/20' })
}
</script>

<template>
  <button @click="send">GetSystemInfo</button>
</template>

<style scoped>
.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
}
.logo:hover {
  filter: drop-shadow(0 0 2em #646cffaa);
}
.logo.vue:hover {
  filter: drop-shadow(0 0 2em #42b883aa);
}
</style>
