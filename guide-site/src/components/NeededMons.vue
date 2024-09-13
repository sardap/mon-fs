<script setup lang="ts">
defineProps<{ neededMons: { species: string; gender: string; count: number }[] }>()
</script>

<template>
  <div>
    <div>
      <div
        v-for="mon in neededMons"
        :key="`${mon.species}_${mon.gender}`"
        class="row-container centered"
      >
        <img class="mon-img" :src="`/gfx/mons/${mon.species.toLowerCase()}.png`" />
        {{ mon.species }}
        <img class="mon-gender" :src="`/gfx/genders/${mon.gender.toLowerCase()}.png`" />
        <p>: {{ mon.count }}</p>
      </div>
    </div>
  </div>
</template>

<style scoped>
div {
  text-align: center;
}

img {
  margin-left: 5px;
  margin-right: 5px;
}

.item-row {
  margin: 10px;
}
</style>
