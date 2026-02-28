export type TemperatureUnit = 'fahrenheit' | 'celsius'
export type SystemOptions = {
	timeZone: string | null
	temperatureUnit: TemperatureUnit
}

const DEFAULT_TEMPERATURE_UNIT: TemperatureUnit = 'fahrenheit'

const getTimeZone = () => {
	return Intl.DateTimeFormat().resolvedOptions().timeZone ?? null
}

export const getSystemOptions = (): SystemOptions => {
	return {
		timeZone: getTimeZone(),
		temperatureUnit: defaultTempUnit(),
	}
}

function defaultTempUnit(): TemperatureUnit {
	return DEFAULT_TEMPERATURE_UNIT
}
