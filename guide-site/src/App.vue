<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import PC from './components/PCView.vue'
import NeededMons from './components/NeededMons.vue'
import { usePcStore } from './stores/pc_store'
import NeededItems from '@/components/NeededItems.vue'
import FreePCSpace from '@/components/FreePCSpace.vue'
import init, { encode_file } from 'mon-fs-box'

const pcStore = usePcStore()

onMounted(async () => {
  await init()
})

const neededItemsAll = computed(() => pcStore.neededItemsAll)
const neededItemsBox = computed(() => pcStore.neededItemsBox)
const neededMonsAll = computed(() => pcStore.neededMonsAll)
const neededMonsBox = computed(() => pcStore.neededMonsBox)
const monCount = computed(() => pcStore.filledMonCount)

const error = ref('')
const loading = ref(false)

const itemsOnlyCurrentBox = ref(true)

function handleBinaryFile(event: Event) {
  if (!event.target) {
    return
  }

  const file: File = (event.target as any).files[0] as File
  const reader = new FileReader()
  loading.value = true

  reader.onload = (e) => {
    const arrayBuffer = e.target?.result as ArrayBuffer
    const uint8Array = new Uint8Array(arrayBuffer)

    error.value = ''

    if (uint8Array.length > 1000000) {
      error.value = 'File too large.'
      loading.value = false
      return
    }

    const pc_json = JSON.stringify({ mons: pcStore.mons })
    console.log('Encoded PC')

    try {
      console.log('Encoding file')
      const json_str = encode_file(pc_json, file.name, uint8Array)
      const pc = JSON.parse(json_str)
      console.log('decoded response')
      pcStore.setMons(pc.mons)
    } catch (e) {
      console.error(e)
      error.value = 'Error encoding file too large.'
    } finally {
      loading.value = false
    }
  }

  reader.readAsArrayBuffer(file)
}

function handlePcFile(event: Event) {
  if (!event.target) {
    return
  }

  const file: File = (event.target as any).files[0] as File
  const reader = new FileReader()
  loading.value = true

  reader.onload = (e) => {
    const arrayBuffer = e.target?.result as ArrayBuffer
    const uint8Array = new Uint8Array(arrayBuffer)

    error.value = ''
    try {
      const json_str = new TextDecoder().decode(uint8Array)
      const pc = JSON.parse(json_str)
      pcStore.setMons(pc.mons)
    } catch (e) {
      console.error(e)
      error.value = 'Error decoding file.'
    } finally {
      loading.value = false
    }
  }

  reader.readAsArrayBuffer(file)
}

function clearPc() {
  pcStore.mons.splice(0, pcStore.mons.length)
  error.value = ''
}

function savePC() {
  let a = window.document.createElement('a')
  a.href = window.URL.createObjectURL(
    new Blob([JSON.stringify({ mons: pcStore.mons })], { type: 'text/json' })
  )
  a.download = 'pc.json'

  document.body.appendChild(a)
  a.click()

  document.body.removeChild(a)
}
</script>

<template>
  <header>
    <h1>Welcome to the PC</h1>
  </header>

  <main>
    <h2>Upload data</h2>
    <p class="file-info">
      Files under 3.2KB are guaranteed to work anything over might compress might not.
    </p>
    <div class="row-container centered file-options">
      <div>
        <p>Upload existing PC guide file</p>
        <input type="file" id="fileInput" accept=".json" @change="(file) => handlePcFile(file)" />
      </div>
      <div>
        <p>Add a file to existing PC</p>
        <input type="file" id="fileInput" @change="(file) => handleBinaryFile(file)" />
      </div>
      <div>
        <p>Clear PC</p>
        <button @click="clearPc()">Clear</button>
      </div>
    </div>
    <p class="error" v-if="error">{{ error }}</p>
    <hr />
    <div>
      <FreePCSpace :mon-count="monCount()" />
    </div>
    <div class="pc-empty" v-if="loading">Crunching the numbers</div>
    <div v-else-if="pcStore.mons.length > 0" class="wrapper">
      <PC />
      <div>
        <h2>Need List</h2>
        <div class="item-box-selector">
          <input
            id="items-current-box"
            name="items-current-box"
            type="checkbox"
            v-model="itemsOnlyCurrentBox"
          />
          <label for="items-current-box">Only show items for current box</label>
        </div>
        <div class="list-wrapper">
          <div>
            <NeededItems v-if="!itemsOnlyCurrentBox" :neededItems="neededItemsAll()" />
            <NeededItems v-else :neededItems="neededItemsBox()" />
          </div>
          <div>
            <NeededMons v-if="!itemsOnlyCurrentBox" :neededMons="neededMonsAll()" />
            <NeededMons v-else :neededMons="neededMonsBox()" />
          </div>
        </div>
      </div>
    </div>
    <div v-else class="pc-empty">
      <p>PC is empty</p>
    </div>
    <hr />
    <div>
      <button class="save-button" :disabled="pcStore.mons.length === 0" @click="savePC">
        Save to the Big PC
      </button>
    </div>
    <br />
    <div>
      <h2>Decoding</h2>
      <p>To decode a mon-fs you will need to follow the guide on the repo linked on my github.</p>
    </div>
  </main>

  <footer>
    <p>THIS IS NOT ENDORSED OR ASSOCIATED WITH ANY COMPANY IN ANYWAY</p>
    <p>
      Created by <a target="_blank" href="https://sarda.dev">Paul Sarda </a>
      <a target="_blank">Source code here</a>
    </p>
  </footer>
</template>

<style scoped>
header {
  text-align: center;
}

main {
  text-align: center;
  padding-left: 2rem;
  padding-right: 2rem;
  min-height: 1200px;
}

hr {
  display: block;
  height: 5px;
  border: 0;
  border-top: 5px solid #dfc5c4;
  margin: 1em 0;
  padding: 0;
}

.wrapper {
  display: grid;
  grid-template-columns: 900px auto;
}

.list-wrapper {
  display: grid;
  grid-template-columns: 50% 50%;
}

@media (max-width: 1200px) {
  .wrapper {
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
  }
}

.save-button {
  font-size: medium;
  padding: 10px;
}

.item-box-selector {
  font-size: large;
}

.item-box-selector input {
  width: 20px;
  height: 20px;
}

.file-info {
  margin-bottom: 5px;
}

header {
  line-height: 1.5;
}

.pc-empty {
  display: flex;
  place-items: center;
  place-content: center;
  height: 800px;
}

.error {
  color: darkred;
  font-weight: bold;
  font-size: large;
}

.file-options div {
  margin-left: 20px;
  margin-right: 20px;
}

.file-options p {
  font-weight: bold;
}

footer {
  bottom: 0;
  width: 100%;
  background-color: #edd37f;
  border-top: 5px solid #bda865;
  padding: 10px;
  text-align: center;
}
</style>
