export const MAX_MON_COUNT = 420

export interface BoxMon {
  species: string
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
