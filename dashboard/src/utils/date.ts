import { format, parseISO, isValid, differenceInYears } from 'date-fns'

export function parseDateSafe(val: string | number | Date | null | undefined): Date | null {
  if (!val) return null
  if (val instanceof Date) {
    return isValid(val) ? val : null
  }
  if (typeof val === 'number') {
    const d = new Date(val)
    return isValid(d) ? d : null
  }
  if (typeof val === 'string') {
    const clean = val.trim()
    if (!clean) return null

    if (/^\d{4}-\d{2}-\d{2}$/.test(clean)) {
      const [year, month, day] = clean.split('-').map(Number)
      const d = new Date(year, month - 1, day)
      return isValid(d) ? d : null
    }

    if (/^\d{4}\/\d{2}\/\d{2}$/.test(clean)) {
      const [year, month, day] = clean.split('/').map(Number)
      const d = new Date(year, month - 1, day)
      return isValid(d) ? d : null
    }

    if (/^\d{2}\/\d{2}\/\d{4}$/.test(clean)) {
      const [day, month, year] = clean.split('/').map(Number)
      const d = new Date(year, month - 1, day)
      return isValid(d) ? d : null
    }

    const isoDate = parseISO(clean)
    if (isValid(isoDate)) return isoDate

    const fallbackDate = new Date(clean)
    if (isValid(fallbackDate)) return fallbackDate
  }
  return null
}

export function formatDate(val: string | number | Date | null | undefined, fallback: string = '-'): string {
  if (!val) return fallback
  if (typeof val === 'string') {
    const clean = val.trim()
    if (/^\d{2}\/\d{2}\/\d{4}$/.test(clean)) {
      return clean
    }
  }

  const d = parseDateSafe(val)
  if (!d) return fallback
  return format(d, 'dd/MM/yyyy')
}

export function calculateAge(fechaNac: string | Date | null | undefined): number | null {
  const d = parseDateSafe(fechaNac)
  if (!d) return null
  const now = new Date()
  const age = differenceInYears(now, d)
  return age >= 0 ? age : null
}

export function formatDayMonth(val: string | number | Date | null | undefined, fallback: string = '-'): string {
  const d = parseDateSafe(val)
  if (!d) return fallback
  return format(d, 'dd/MM')
}
