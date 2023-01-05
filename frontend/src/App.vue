<script setup lang="ts">
import { invoke, ActionType } from './bridge';
const send = () => {
  invoke(ActionType.GetSystemInfo, { callback: (...params) => {
    console.log('GetSystemInfo', ...params);
  } })
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
