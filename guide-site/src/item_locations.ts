const itemLocations = [
  {
    name: 'SlateportCity Mart Pokemart',
    items: [
      'Poké Ball',
      'Great Ball',
      'Potion',
      'Super Potion',
      'Antidote',
      'Parlyz Heal',
      'Escape Rope',
      'Repel',
      'HARBOR MAIL'
    ]
  },
  {
    name: 'PetalburgCity Mart Pokemart',
    items: [
      'Poké Ball',
      'Great Ball',
      'Potion',
      'Super Potion',
      'Antidote',
      'Parlyz Heal',
      'Awakening',
      'Escape Rope',
      'Repel',
      'X Speed',
      'X Attack',
      'X Defend',
      'ORANGE MAIL'
    ]
  },
  {
    name: 'FortreeCity Mart Pokemart',
    items: [
      'Great Ball',
      'ULTRA BALL',
      'Super Potion',
      'Hyper Potion',
      'Antidote',
      'Parlyz Heal',
      'Awakening',
      'Revive',
      'Super Repel',
      'WOOD MAIL'
    ]
  },
  {
    name: 'SlateportCity Pokemart EnergyGuru',
    items: ['PROTEIN', 'IRON', 'CARBOS', 'ZINC', 'CALCIUM', 'HP UP']
  },
  {
    name: 'SlateportCity Pokemart PowerTMs',
    items: ['TM10 @ Hidden Power', 'TM43 @ Secret Power']
  },
  {
    name: 'EverGrandeCity PokemonLeague 1F Pokemart',
    items: [
      'ULTRA BALL',
      'Hyper Potion',
      'Max Potion',
      'Full Restore',
      'Full Heal',
      'Revive',
      'Max Repel'
    ]
  },
  {
    name: 'MauvilleCity Mart Pokemart',
    items: [
      'Great Ball',
      'Super Potion',
      'Antidote',
      'Parlyz Heal',
      'Awakening',
      'X Speed',
      'X Attack',
      'X Defend',
      'Guard Spec.',
      'Dire Hit',
      'X Accuracy'
    ]
  },
  {
    name: 'LavaridgeTown HerbShop Pokemart',
    items: ['Energypowder', 'Energy Root', 'Heal Powder', 'Revival Herb']
  },
  {
    name: 'LavaridgeTown Mart Pokemart',
    items: [
      'Great Ball',
      'Super Potion',
      'Antidote',
      'Parlyz Heal',
      'Awakening',
      'Burn Heal',
      'Revive',
      'Super Repel',
      'X Speed'
    ]
  },
  {
    name: 'MossdeepCity Mart Pokemart',
    items: [
      'ULTRA BALL',
      'NET BALL',
      'DIVE BALL',
      'Hyper Potion',
      'Full Heal',
      'Revive',
      'Max Repel',
      'X Attack',
      'X Defend'
    ]
  },
  {
    name: 'FallarborTown Mart Pokemart',
    items: [
      'Great Ball',
      'Super Potion',
      'Antidote',
      'Parlyz Heal',
      'Escape Rope',
      'Super Repel',
      'X Special',
      'X Speed',
      'X Attack',
      'X Defend',
      'Dire Hit',
      'Guard Spec.'
    ]
  },
  {
    name: 'SootopolisCity Mart Pokemart',
    items: [
      'ULTRA BALL',
      'Hyper Potion',
      'Max Potion',
      'Full Heal',
      'Revive',
      'Max Repel',
      'X Attack',
      'X Defend',
      'SHADOW MAIL'
    ]
  },
  {
    name: 'VerdanturfTown Mart Pokemart',
    items: [
      'Great Ball',
      'Nest Ball',
      'Super Potion',
      'Antidote',
      'Parlyz Heal',
      'Awakening',
      'Burn Heal',
      'Ice Heal',
      'Repel',
      'X Special',
      'Fluffy Tail'
    ]
  },
  {
    name: 'OldaleTown Mart Pokemart',
    items: ['Poké Ball', 'Potion', 'Antidote', 'Parlyz Heal', 'Awakening']
  },
  {
    name: 'RustboroCity Mart Pokemart',
    items: [
      'Poké Ball',
      'Timer Ball',
      'REPEAT BALL',
      'Potion',
      'Super Potion',
      'Antidote',
      'Parlyz Heal',
      'Escape Rope',
      'Repel',
      'X Speed',
      'X Attack',
      'X Defend'
    ]
  }
]

export function getLocationsForItems(items: string[]): Map<string, string> {
  const result: Map<string, string> = new Map()

  const martTally: Map<string, number> = new Map()
  const locationMapping: Map<string, string[]> = new Map()

  items.forEach((item) => {
    if (!locationMapping.has(item)) {
      locationMapping.set(item, [])
    }
    itemLocations.forEach((store) => {
      store.items.forEach((storeItem) => {
        if (storeItem === item) {
          locationMapping.get(item)!.push(store.name)
          martTally.set(store.name, (martTally.get(store.name) || 0) + 1)
        }
      })
    })
  })

  const selectedLocations = new Set<string>()
  items.forEach((item) => {
    const locations = locationMapping.get(item)!
    let bestLocation: string | null = null
    let bestScore: number = Number.MIN_SAFE_INTEGER

    for (const location of locations) {
      const count = martTally.get(location)!
      if (count > bestScore) {
        bestScore = count
        bestLocation = location
      }
    }

    selectedLocations.add(bestLocation!)
  })
  items.forEach((item) => {
    result.set(
      item,
      locationMapping.get(item)!.find((location) => selectedLocations.has(location))!
    )
  })

  return result
}
