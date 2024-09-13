<script setup lang="ts">
import { type BoxMon, type WebBoxMon } from '@/pc'
import { usePcStore } from '@/stores/pc_store'
import { ref } from 'vue'

const pcStore = usePcStore()

const props = defineProps<{ boxNumber: number }>()

const offset = props.boxNumber * 30

const width = 6
const height = 5

const visibleMons = ref<WebBoxMon[]>(
  pcStore.mons.slice(offset, offset + width * height).filter((mon) => mon)
)

function imageForMon(mon: BoxMon) {
  return `/gfx/mons/${mon.species.toLowerCase()}.png`
}

function imageForItem(item: string) {
  return `/gfx/items/${item.replace('.', '-')}.png`
}

function itemClicked(mon: WebBoxMon) {
  if (mon.caught && mon.held_item !== '') {
    mon.holding_item = !mon.holding_item
  }
}

function monClicked(mon: WebBoxMon) {
  mon.caught = !mon.caught
  if (mon.held_item !== '') {
    mon.holding_item = false
  }
}

function monItemClass(mon: WebBoxMon) {
  let result = ''
  if (mon.caught && mon.held_item !== '') {
    result += 'clickable'
  }

  if (mon.holding_item) {
    result += ' part-complete'
  } else {
    result += ' part-incomplete'
  }

  return result
}
</script>

<template>
  <div class="grid-container">
    <div
      v-for="mon in visibleMons"
      :key="mon.index"
      :class="`box-mon ` + (mon.caught && mon.holding_item ? `caught` : `uncaught`)"
    >
      <div v-if="mon">
        <div
          @click="monClicked(mon)"
          :class="`clickable ` + (mon.caught ? `part-complete` : `part-incomplete`)"
        >
          <p class="mon-name">
            {{ mon.name }}
            <img
              class="mon-gender"
              :src="`/gfx/genders/${mon.gender.toLowerCase()}.png`"
              :alt="mon.gender"
            />
          </p>
          <img :src="imageForMon(mon)" :alt="mon.name" class="mon-img" />
        </div>
        <div class="item-part" @click="itemClicked(mon)" :class="monItemClass(mon)">
          <p>{{ mon.held_item }}</p>
          <img v-if="mon.held_item" :src="imageForItem(mon.held_item)" />
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.mon-name {
  font-family: 'Courier New', Courier, monospace;
}

.grid-container {
  display: grid;
  grid-template-columns: repeat(6, 150px);
  gap: 0px;
}

.clickable:hover {
  cursor: pointer;
}

.part-complete {
  background-color: lightgreen;
}

.clickable.part-complete:hover {
  background-color: lightcoral;
}

.caught {
  border: 5px solid green;
}

.uncaught {
  border: 5px solid red;
}

.clickable.part-incomplete:hover {
  background-color: lightgray;
}

.item-part {
  height: 54px;
}

.box-mon {
  text-align: center;
  width: 100%;
}
</style>
