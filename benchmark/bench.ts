import b from 'benny'

import { DisplayManager } from '../index'

async function run() {
  await b.suite(
    'List displays',

    b.add('Native list displays', () => {
      const displays = new DisplayManager().list()
      for (const display of displays) {
        console.info(display) // Print display data on the console
      }
    }),

    b.cycle(),
    b.complete(),
  )
}

run().catch((e) => {
  console.error(e)
})
