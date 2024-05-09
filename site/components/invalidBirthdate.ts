export function calculateMinDate() {
  const currentDate = new Date()
  const maximumAge = 18
  currentDate.setFullYear(currentDate.getFullYear() - maximumAge)
  return currentDate
}

export default function invalidBirthdate(date: string) {
  const minDate = calculateMinDate()
  if (minDate < new Date(date)) return true
  return false
}
