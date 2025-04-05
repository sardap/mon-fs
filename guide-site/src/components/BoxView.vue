<script setup lang="ts">
import { type WebBoxMon } from '@/pc'
import { usePcStore } from '@/stores/pc_store'
import { ref } from 'vue'
import { getItemIcon, getPcMarkIcon, boolBoxMonToGenderIcon } from '@/icons'

const pcStore = usePcStore()

const props = defineProps<{ boxNumber: number }>()

const offset = props.boxNumber * 30

const width = 6
const height = 5

const visibleMons = ref<WebBoxMon[]>(
  pcStore.mons.slice(offset, offset + width * height).filter((mon) => mon)
)

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
  <div class="pc-grid-container">
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
              :src="`gfx/genders/${mon.gender.toLowerCase()}.png`"
              :alt="mon.gender"
            />
          </p>
          <p v-if="pcStore.sizeMode() == `full`">{{ mon.shiny ? `✨ ` : `` }}{{ mon.species }}</p>
          <img v-if="mon.ball" :src="getItemIcon(mon.ball)" />
          <img :src="`gfx/mons/${mon.species.toLowerCase()}.png`" :alt="mon.name" class="mon-img" />
          <p v-if="mon.virus !== undefined">Pokerus:{{ mon.virus ? `😷YES` : `NO` }}</p>
          <div v-if="mon.pc_mark">
            <img
              style="margin: 0 2px"
              width="15"
              :src="`gfx/pc_mark/${getPcMarkIcon(mon.pc_mark[0], 0)}`"
            />
            <img
              style="margin: 0 2px"
              width="15"
              :src="`gfx/pc_mark/${getPcMarkIcon(mon.pc_mark[2], 1)}`"
            />
            <img
              style="margin: 0 2px"
              width="15"
              :src="`gfx/pc_mark/${getPcMarkIcon(mon.pc_mark[1], 2)}`"
            />
            <img
              style="margin: 0 2px"
              width="15"
              :src="`gfx/pc_mark/${getPcMarkIcon(mon.pc_mark[3], 3)}`"
            />
          </div>
          <p v-if="mon.exp != undefined">EXP: {{ mon.exp }}</p>
          <p v-if="mon.ribbons != undefined">Ribbons: {{ mon.ribbons }}</p>
          <div v-if="mon.ot_tid != undefined" class="ot-info">
            <p style="text-align: center">
              OT
              <img
                v-if="mon.ot_gender !== undefined"
                class="mon-gender"
                :src="boolBoxMonToGenderIcon(mon.ot_gender)"
                :alt="mon.gender"
              />
            </p>
            <p v-if="mon.ot_name">{{ mon.ot_name }}</p>
            <p>TID: {{ mon.ot_tid }}</p>
            <p v-if="mon.met_level !== undefined">MET@: {{ mon.met_level }}</p>
          </div>
          <div v-if="mon.move_set">
            <p style="text-align: center">Moves</p>
            <p class="move-set" v-for="move in mon.move_set" :key="move">{{ move }}</p>
          </div>
          <br />
        </div>
        <div class="item-part" @click="itemClicked(mon)" :class="monItemClass(mon)">
          <p>{{ mon.held_item }}</p>
          <img v-if="mon.held_item" :src="getItemIcon(mon.held_item)" />
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.move-set {
  text-align: left;
  margin-left: 10px;
}

.mon-name {
  font-family: 'Courier New', Courier, monospace;
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

.ot-info {
  text-align: center;
  margin-left: 10px;
  margin-top: 5px;
  margin-bottom: 5px;
}
</style>
