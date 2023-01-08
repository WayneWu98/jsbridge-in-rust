<script setup lang="ts">
import * as BridgeAPI from './bridge'

const getSystemInfo = () => {
  BridgeAPI
    .getSystemInfo()
    .then(console.log)
}
const download = () => {
  BridgeAPI
    .download({ 
      url: 'https://plus.unsplash.com/premium_photo-1666976923529-f0366d02d726?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=3871&q=80',
      onProgress: (params) => console.log('onProgress', params),
      onSuccess: () => console.log('onSuccess'),
      onFinally: () => console.log('onFinally'),
    })
}
</script>

<template>
  <button @click="getSystemInfo">GetSystemInfo</button>
  <button @click="download">Download</button>
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
