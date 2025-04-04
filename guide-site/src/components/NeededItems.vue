<script setup lang="ts">
import { computed } from 'vue'
import { getLocationsForItems } from '../item_locations'
import { usePcStore } from '@/stores/pc_store'
import { getItemIcon } from '@/icons'

const pcStore = usePcStore()

const props = defineProps<{ neededItems: { name: string; count: number }[] }>()

const neededItems = computed(() => {
  if (pcStore.sizeMode() === 'full') {
    return props.neededItems.map((item) => {
      return {
        ...item,
        location: null
      }
    })
  }

  const locations = getLocationsForItems(props.neededItems.map((item) => item.name))

  const result = props.neededItems.map((item) => {
    return {
      ...item,
      location: locations.get(item.name)
    }
  })

  result.sort((a, b) => {
    if (a.location === b.location) {
      return a.name.localeCompare(b.name)
    }

    if (!a.location || !b.location) {
      return 1
    }

    return a.location.localeCompare(b.location)
  })

  return result
})
</script>

<template>
  <div v-for="item in neededItems" :key="item.name" class="item-row">
    <p>
      <img :src="getItemIcon(item.name)" /> {{ item.name }} :
      {{ item.count }}
    </p>
    <p v-if="item.location">{{ item.location }}</p>
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
