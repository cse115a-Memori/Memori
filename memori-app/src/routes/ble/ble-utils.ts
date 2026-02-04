import type { BleDevice } from '@mnlphlp/plugin-blec'

export function displayName(
	device: BleDevice,
	resolvedNames: Record<string, string>
) {
	return device.name || resolvedNames[device.address] || ''
}

export function isRecognizableName(name: string) {
	const trimmed = name.trim()
	if (!trimmed) return false

	const lower = trimmed.toLowerCase()
	if (lower === 'unknown' || lower === 'n/a') return false

	const letters = (trimmed.match(/[a-z]/gi) || []).length
	const vowels = (trimmed.match(/[aeiou]/gi) || []).length
	const digits = (trimmed.match(/\d/g) || []).length
	const separators = (trimmed.match(/[-_:\s]/g) || []).length
	const hasSpace = /\s/.test(trimmed)

	if (letters < 3 || vowels === 0) return false
	if (digits > 0 && digits + separators >= trimmed.length - 1) return false
	if (digits >= letters * 2) return false
	if (/^[0-9a-f:\-]+$/i.test(trimmed) && letters <= 2) return false
	if (/[0-9a-f]{6,}/i.test(trimmed)) return false
	if (/^[0-9a-f]{4,}$/i.test(trimmed)) return false
	if (!hasSpace && trimmed.length >= 8 && vowels <= 1) return false

	return true
}

export function filterAndSortDevices(
	devices: BleDevice[],
	showAllDevices: boolean,
	resolvedNames: Record<string, string>
) {
	const filtered = showAllDevices
		? devices
		: devices.filter((device) =>
				isRecognizableName(displayName(device, resolvedNames))
			)
	return filtered.slice().sort((a, b) => b.rssi - a.rssi)
}

export function signalStrength(rssi: number) {
	if (rssi >= -55) return 'Excellent'
	if (rssi >= -65) return 'Strong'
	if (rssi >= -75) return 'Good'
	if (rssi >= -85) return 'Fair'
	return 'Weak'
}

export function signalClass(rssi: number) {
	if (rssi >= -55) return 'bg-emerald-300'
	if (rssi >= -65) return 'bg-emerald-200'
	if (rssi >= -75) return 'bg-yellow-200'
	if (rssi >= -85) return 'bg-orange-200'
	return 'bg-red-200'
}
