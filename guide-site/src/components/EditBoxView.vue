<script setup lang="ts">
import { type WebBoxMon, POSSIBLE_NAME_LETTERS, POSSIBLE_SPECIES, POSSIBLE_ITEMS } from '@/pc'
import { usePcStore } from '@/stores/pc_store'
import { computed } from 'vue'

const pcStore = usePcStore()

const props = defineProps<{ boxNumber: number }>()

const offset = props.boxNumber * 30

const width = 6
const height = 5

const visibleMons = computed(() => pcStore.mons.slice(offset, offset + width * height))

function cleanName(mon: WebBoxMon) {
  if (mon.name.length > 10) {
    mon.name = mon.name.slice(0, 10)
  }

  mon.name = mon.name
    .split('')
    .filter((char) => POSSIBLE_NAME_LETTERS.includes(char))
    .join('')
}

function addMon() {
  pcStore.addMon()
}

function removeMon() {
  pcStore.removeMon()
}

function invertGender(mon: WebBoxMon) {
  if (mon.gender === 'Male') {
    mon.gender = 'Female'
  } else {
    mon.gender = 'Male'
  }
}
</script>

<template>
  <div class="pc-grid-container">
    <div v-for="mon in visibleMons" :key="mon.index" :class="`box-mon`">
      <div v-if="mon">
        <div>
          <div class="row-container centered">
            <input type="text" v-model="mon.name" @keyup="cleanName(mon)" />
            <button @click="invertGender(mon)">
              <img
                class="mon-gender"
                :src="`gfx/genders/${mon.gender.toLowerCase()}.png`"
                :alt="mon.gender"
              />
            </button>
          </div>
          <select v-model="mon.species">
            <option v-for="species in POSSIBLE_SPECIES" :key="species" :value="species">
              {{ species.toLowerCase() }}
            </option>
          </select>
          <img
            :key="mon.species"
            :src="`gfx/mons/${mon.species.toLowerCase()}.png`"
            :alt="mon.species"
            class="mon-img"
          />
        </div>
        <select v-model="mon.held_item">
          <option v-for="item in POSSIBLE_ITEMS" :key="item" :value="item">
            {{ item }}
          </option>
        </select>
        <img v-if="mon.held_item" :src="`gfx/items/${mon.held_item.replace('.', '-')}.png`" />
      </div>
    </div>
    <div v-if="visibleMons.length < 30" class="box-mon add-remove">
      <button @click="removeMon">Delete</button>
      <br />
      <button @click="addMon">Add</button>
    </div>
  </div>
</template>

<style scoped>
input {
  width: 90px;
  margin-right: 5px;
  margin-bottom: 5px;
}

select {
  margin-bottom: 5px;
  width: 90%;
}

.add-remove {
  border: 5px solid darkgoldenrod;
}

.add-remove button {
  width: 100px;
  height: 50px;
  margin: 10px;
  font-size: medium;
}

.mon-gender:hover {
  cursor: pointer;
}
</style>
