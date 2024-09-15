export const MAX_MON_COUNT = 420
export const BOX_COUNT = 14
export const MONS_PER_BOX = MAX_MON_COUNT / BOX_COUNT

export const POSSIBLE_NAME_LETTERS =
  'aAbBcCdDeEfFgGhHiIjJkKmMnNoOpPqQrRsStTuUvVwWxXyYzZ23456789!?/-…♂♀'

export const POSSIBLE_SPECIES = ['POOCHYENA', 'NINCADA', 'WHISMUR', 'TAILLOW']

export const POSSIBLE_ITEMS = [
  'Antidote',
  'Awakening',
  'Burn Heal',
  'Dire Hit',
  'Energy Powder',
  'Energy Root',
  'Escape Rope',
  'Fluffy Tail',
  'Full Heal',
  'Full Restore',
  'Great Ball',
  'Guard Spec',
  'Heal Powder',
  'Hyper Potion',
  'Ice Heal',
  'Max Potion',
  'Max Repel',
  'Parlyz Heal',
  'Poke Ball',
  'Potion',
  'Protein',
  'Repel',
  'Revival Herb',
  'Revive',
  'Super Potion',
  'Super Repel',
  'Timer Ball',
  'X Accuracy',
  'X Attack',
  'X Defend',
  'X Special',
  'X Speed'
]

export interface BoxMon {
  species: 'POOCHYENA' | 'NINCADA' | 'WHISMUR' | 'TAILLOW'
  gender: 'Male' | 'Female'
  name: string
  held_item: string
}

export interface PC {
  mons: (BoxMon | null)[]
}

export interface WebBoxMon extends BoxMon {
  caught: boolean
  holding_item: boolean
  index: number
}

export interface WebPC {
  mons: (WebBoxMon | null)[]
}
