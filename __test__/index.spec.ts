import test from 'ava'

import { DisplayManager } from '../index'

test('list every displays available', () => {
  const displays = new DisplayManager().list()
  for (const display of displays) {
    console.info(display) // Print display data on the console
  }
})
