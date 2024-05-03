import { styleTags, tags } from '@lezer/highlight'

export const highlighting = styleTags({
  Identifer: tags.name,
  Number: tags.number,
  String: tags.string
})
