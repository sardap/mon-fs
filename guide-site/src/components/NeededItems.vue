<script setup lang="ts">
defineProps<{ neededItems: { name: string; count: number }[] }>()
</script>

<template>
  <div v-for="item in neededItems" :key="item.name" class="item-row">
    <p>
      <img :src="`/gfx/items/${item.name.replace('.', '-')}.png`" /> {{ item.name }} :
      {{ item.count }}
    </p>
  </div>
</template>

<style scoped>
div {
  text-align: center;
}

.item-row {
  margin: 10px;
}
</style>
