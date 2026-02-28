import type { ResultAsync } from 'neverthrow'
import { type AppError, commands, type MemoriWidget, tryCmd } from '@/tauri'

export function getWidgetKinds(): ResultAsync<MemoriWidget[], AppError> {
	return tryCmd(commands.getWidgetKinds()).map(data => [...data])
}

// export function isValidWidgetId() {

// }
