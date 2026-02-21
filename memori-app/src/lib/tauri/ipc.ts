import { errAsync, okAsync, ResultAsync } from 'neverthrow'
import type { Result as SpectaResult } from './bindings'

export type AppError = string

export function toCmdError(e: unknown): AppError {
	return typeof e === 'string' ? e : e instanceof Error ? e.message : String(e)
}

export function tryCmd<T>(
	p: Promise<SpectaResult<T, AppError>>
): ResultAsync<T, AppError> {
	return ResultAsync.fromPromise(p, toCmdError).andThen(r =>
		r.status === 'ok' ? okAsync(r.data) : errAsync(toCmdError(r.error))
	)
}
