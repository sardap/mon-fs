import { BOX_COUNT, type BoxMon, type WebBoxMon } from '@/pc'
import { defineStore } from 'pinia'
import { ref } from 'vue'

export const usePcStore = defineStore('pc', () => {
  const mons = ref<WebBoxMon[]>([])
  const currentBox = ref(0)

  function setMons(new_mons: (BoxMon | WebBoxMon | null)[]) {
    currentBox.value = 0
    mons.value = []

    new_mons.forEach((mon, i) => {
      if (!mon) {
        return
      }

      mons.value.push({
        ...mon,
        caught: 'caught' in mon ? mon.caught : false,
        holding_item: 'holding_item' in mon ? mon.holding_item : mon.held_item === '',
        index: i
      } as WebBoxMon)
    })
  }

  function pcJson() {
    return JSON.stringify({ mons: mons.value })
  }

  function neededItemsAll() {
    const tally: Record<string, { count: number; first_index: number }> = {}

    for (const mon of mons.value) {
      if (!mon) continue
      if (mon.held_item && !mon.holding_item) {
        if (tally[mon.held_item]) {
          tally[mon.held_item].count += 1
        } else {
          tally[mon.held_item] = {
            count: 1,
            first_index: mon.index
          }
        }
      }
    }

    const result = []

    for (const item in tally) {
      result.push({ name: item, count: tally[item].count })
    }

    result.sort((a, b) => a.name.localeCompare(b.name))

    return result
  }

  function neededItemsBox() {
    const tally: Record<string, { count: number; first_index: number }> = {}

    for (let i = currentBox.value * 30; i < currentBox.value * 30 + 30; i++) {
      const mon = mons.value[i]
      if (!mon) continue
      if (mon.held_item && !mon.holding_item) {
        if (tally[mon.held_item]) {
          tally[mon.held_item].count += 1
        } else {
          tally[mon.held_item] = {
            count: 1,
            first_index: mon.index
          }
        }
      }
    }

    const result = []

    for (const item in tally) {
      result.push({ name: item, count: tally[item].count })
    }

    result.sort((a, b) => a.name.localeCompare(b.name))

    return result
  }

  function neededMonsAll() {
    const tally: Record<string, { mon: BoxMon; count: number }> = {}
    mons.value.forEach((mon) => {
      if (!mon.caught) {
        const key = `${mon.gender}_${mon.species}`

        if (key in tally) {
          tally[key].count += 1
        } else {
          tally[key] = { mon: mon, count: 1 }
        }
      }
    })

    const result: {
      gender: 'Male' | 'Female'
      species: string
      count: number
    }[] = []

    for (const key in tally) {
      result.push({
        gender: tally[key].mon.gender,
        species: tally[key].mon.species,
        count: tally[key].count
      })
    }

    result.sort((a, b) => {
      if (a.species === b.species) {
        return a.gender.localeCompare(b.gender)
      }

      return a.species.localeCompare(b.species)
    })

    return result
  }

  function neededMonsBox() {
    const tally: Record<string, { mon: BoxMon; count: number }> = {}
    for (let i = currentBox.value * 30; i < currentBox.value * 30 + 30; i++) {
      const mon = mons.value[i]
      if (mon && !mon.caught) {
        const key = `${mon.gender}_${mon.species}`

        if (key in tally) {
          tally[key].count += 1
        } else {
          tally[key] = { mon: mon, count: 1 }
        }
      }
    }

    const result: {
      gender: 'Male' | 'Female'
      species: string
      count: number
    }[] = []

    for (const key in tally) {
      result.push({
        gender: tally[key].mon.gender,
        species: tally[key].mon.species,
        count: tally[key].count
      })
    }

    result.sort((a, b) => {
      if (a.species === b.species) {
        return a.gender.localeCompare(b.gender)
      }

      return a.species.localeCompare(b.species)
    })

    return result
  }

  function toggleCaught(index: number) {
    mons.value[index].caught = !mons.value[index].caught
  }

  function filledMonCount() {
    return mons.value.length
  }

  function addMon() {
    mons.value.push({
      species: 'POOCHYENA',
      gender: 'Male',
      name: 'aaaaaaaaaa',
      held_item: '',
      caught: false,
      holding_item: false,
      index: mons.value.length
    })
  }

  function removeMon() {
    mons.value.pop()
  }

  function lastBox() {
    return Math.min(
      Math.ceil(mons.value.length / 30) + Number(mons.value.length % 30 == 0),
      BOX_COUNT
    )
  }

  return {
    mons,
    currentBox,
    neededItemsAll,
    neededItemsBox,
    neededMonsAll,
    neededMonsBox,
    filledMonCount,
    setMons,
    toggleCaught,
    addMon,
    removeMon,
    lastBox,
    pcJson
  }
})
