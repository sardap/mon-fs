<script setup lang="ts">
import { MAX_MON_COUNT } from '@/pc'

defineProps<{ monCount: number }>()
</script>

<template>
  <p>
    {{ Math.round((monCount / MAX_MON_COUNT) * 100) }}% of PC slots filled
    {{ MAX_MON_COUNT - monCount }} slots of {{ MAX_MON_COUNT }} slots remain
  </p>
</template>

<style scoped>
p {
  font-size: large;
}
</style>
