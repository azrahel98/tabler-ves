import { format, isValid, parseISO } from 'date-fns'

export function formatearFecha(fecha: Date | string | number | null | undefined, patron = 'dd/MM/yyyy'): string {
	if (!fecha) return ''

	let fechaObj: Date

	if (fecha instanceof Date) {
		fechaObj = fecha
	} else if (typeof fecha === 'number') {
		fechaObj = new Date(fecha)
	} else if (typeof fecha === 'string') {
		const textoLimpio = fecha.trim()
		if (!textoLimpio) return ''

		if (/^\d{4}-\d{2}-\d{2}$/.test(textoLimpio)) {
			const [anio, mes, dia] = textoLimpio.split('-').map(Number)
			fechaObj = new Date(anio, mes - 1, dia)
		} else {
			fechaObj = parseISO(textoLimpio)
			if (!isValid(fechaObj)) {
				fechaObj = new Date(textoLimpio)
			}
		}
	} else {
		return ''
	}

	return isValid(fechaObj) ? format(fechaObj, patron) : ''
}
