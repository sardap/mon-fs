<script setup lang="ts">
import {
  type WebBoxMon,
  POSSIBLE_NAME_LETTERS,
  POSSIBLE_SPECIES,
  POSSIBLE_ITEMS,
  POSSIBLE_SPECIES_FULL,
  POSSIBLE_BALLS,
  POSSIBLE_MOVES_FULL,
  POSSIBLE_ITEMS_FULL
} from '@/pc'
import { usePcStore } from '@/stores/pc_store'
import { computed } from 'vue'
import { getItemIcon, getPcMarkIcon, boolBoxMonToGenderIcon } from '@/icons'

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

const speciesList = computed(() => {
  return pcStore.sizeMode() === 'lite' ? POSSIBLE_SPECIES : POSSIBLE_SPECIES_FULL
})
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
            <option v-for="species in speciesList" :key="species" :value="species">
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
        <div>
          <select v-model="mon.ball">
            <option v-for="item in POSSIBLE_BALLS" :key="item" :value="item">
              {{ item }}
            </option>
          </select>
          <img v-if="mon.ball" :src="getItemIcon(mon.ball)" />
        </div>
        <div v-if="pcStore.sizeMode() === 'full'">
          <label>Pokerus: </label>
          <input type="checkbox" v-model="mon.virus" />
        </div>
        <div v-if="pcStore.sizeMode() === 'full'">
          <label>Shiny: </label>
          <input type="checkbox" v-model="mon.shiny" />
        </div>
        <div v-if="mon.pc_mark">
          <img
            class="pc-mark"
            v-for="(mark, i) in mon.pc_mark"
            :key="i"
            @click="
              () => {
                if (!mon.pc_mark) {
                  mon.pc_mark = []
                }

                mon.pc_mark[i] = !mon.pc_mark[i]
              }
            "
            :src="`/gfx/pc_mark/${getPcMarkIcon(mark, i)}`"
            width="25"
          />
        </div>
        <div>
          <label>EXP: </label>
          <input type="number" v-model="mon.exp" />
        </div>
        <div v-if="pcStore.sizeMode() === 'full'">
          <p>
            OT
            <button v-if="mon.ot_gender !== undefined" @click="mon.ot_gender = !mon.ot_gender">
              <img
                class="mon-gender"
                :src="boolBoxMonToGenderIcon(mon.ot_gender)"
                :alt="mon.gender"
              />
            </button>
          </p>
          <label>Name</label>
          <input v-model="mon.ot_name" />
          <label>TID</label>
          <input class="ot-tid" type="number" v-model="mon.ot_tid" />
          <label>Met@</label>
          <input class="met-level" type="number" v-model="mon.met_level" />
        </div>
        <div v-if="mon.move_set">
          <p>Moves</p>
          <div v-for="(move, i) in mon.move_set" :key="i">
            <select v-model="mon.move_set[i]">
              <option v-for="move in POSSIBLE_MOVES_FULL" :key="move" :value="move">
                {{ move }}
              </option>
            </select>
          </div>
        </div>
        <br />
        <div v-if="pcStore.sizeMode() === 'lite'">
          <select v-model="mon.held_item">
            <option v-for="item in POSSIBLE_ITEMS" :key="item" :value="item">
              {{ item }}
            </option>
          </select>
        </div>
        <div v-else>
          <select v-model="mon.held_item">
            <option v-for="item in POSSIBLE_ITEMS_FULL" :key="item" :value="item">
              {{ item }}
            </option>
          </select>
        </div>
        <img v-if="mon.held_item" :src="getItemIcon(mon.held_item)" />
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

.ot-tid {
  width: 80px;
}

.met-level {
  width: 50px;
}

input[type='checkbox'] {
  width: 20px;
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

.pc-mark {
  padding: 5px;
}

.pc-mark:hover {
  cursor: pointer;
}
</style>
