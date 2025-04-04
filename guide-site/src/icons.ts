export function getItemIcon(item: string) {
  const itemUpper = item.toUpperCase().replace(' ', '').replace('.', '').replace('É', 'E')

  if (itemUpper === 'PARLYZHEAL') {
    return 'gfx/items/PARALYZEHEAL.png'
  }

  if (itemUpper.startsWith('TM')) {
    return `gfx/items/TM01.png`
  }

  return `gfx/items/${itemUpper.replace('.', '-')}.png`
}

export function getPcMarkIcon(value: boolean, index: number) {
  switch (index) {
    case 0:
      return value ? 'pc_mark_circle_white.png' : 'pc_mark_circle_black.png'
    case 1:
      return value ? 'pc_mark_square_white.png' : 'pc_mark_square_black.png'
    case 2:
      return value ? 'pc_mark_tri_white.png' : 'pc_mark_tri_black.png'
    case 3:
      return value ? 'pc_mark_heart_white.png' : 'pc_mark_heart_black.png'
  }

  return ''
}

export function boolBoxMonToGenderIcon(value: boolean) {
  return `gfx/genders/${value ? 'female' : 'male'}.png`
}
