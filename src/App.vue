<script setup lang="ts">
import Envconfig from './components/envconfig.vue'
</script>

<template>
    <Envconfig />
</template>


<style>
/* 隐藏 webkit 浏览器的滚动条 */
::-webkit-scrollbar {
  display: none;
}

/* 针对 Firefox */
* {
  scrollbar-width: none;
}
/* 清除一下边距 */
body {
  margin: 0;
  padding: 0;
 
}

</style>