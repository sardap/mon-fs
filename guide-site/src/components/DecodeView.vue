<script setup lang="ts">
import { usePcStore } from '@/stores/pc_store'
import EditBoxView from './EditBoxView.vue'
import { computed, ref } from 'vue'
import { decode_file } from 'mon-fs-web-box'

const pcStore = usePcStore()
const endBox = computed(() => pcStore.lastBox())
const error = ref('')

function changeBox(delta: number) {
  pcStore.currentBox += delta
}

function decodeButton() {
  error.value = ''
  const pc_json = JSON.stringify({ mons: pcStore.mons })
  let raw: Uint8Array
  try {
    raw = decode_file(pc_json)
  } catch (e) {
    console.error(e)
    error.value = 'Error decoding input check everything is correct'
    return
  }

  let a = window.document.createElement('a')
  a.href = window.URL.createObjectURL(new Blob([raw], { type: 'text/json' }))
  a.download = 'files.zip'

  document.body.appendChild(a)
  a.click()

  document.body.removeChild(a)
}
</script>

<template>
  <div>
    <p>Enter EXACTLY what you see in the data portion of the PC</p>
    <hr />
    <div class="wrapper">
      <div class="box-header">
        <h2>
          <button :disabled="pcStore.currentBox - 1 < 0" @click="changeBox(-1)">PREVIOUS</button>BOX
          {{ pcStore.currentBox + 1
          }}<button :disabled="pcStore.currentBox + 1 >= endBox" @click="changeBox(1)">NEXT</button>
        </h2>
      </div>
      <EditBoxView
        :key="pcStore.currentBox"
        :box-number="pcStore.currentBox"
        :mons="pcStore.mons"
      />
    </div>
    <hr />
    <div>
      <p class="error" v-if="error">{{ error }}</p>
      <button :onclick="decodeButton">Decode and download</button>
    </div>
    <br />
  </div>
</template>

<style scoped>
.wrapper {
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;
}
</style>
