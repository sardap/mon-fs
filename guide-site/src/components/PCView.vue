<script setup lang="ts">
import BoxView from './BoxView.vue'
import { usePcStore } from '@/stores/pc_store'

const pcStore = usePcStore()

const endBox = Math.ceil(
  pcStore.mons.map((mon) => (mon ? (1 as number) : 0)).reduce((a, b) => a + b, 0) / 30
)

function changeBox(delta: number) {
  pcStore.currentBox += delta
}
</script>

<template>
  <div class="pc-view">
    <div class="box-header">
      <h2>
        <button :disabled="pcStore.currentBox - 1 < 0" @click="changeBox(-1)">PREVIOUS</button>BOX
        {{ pcStore.currentBox + 1
        }}<button :disabled="pcStore.currentBox + 1 >= endBox" @click="changeBox(1)">NEXT</button>
      </h2>
    </div>
    <BoxView :key="pcStore.currentBox" :box-number="pcStore.currentBox" :mons="pcStore.mons" />
  </div>
</template>

<style scoped></style>
