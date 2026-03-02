export const cardCls = {
	Base: 'w-full min-h-24 rounded-2xl border border-slate-200 bg-gradient-to-br from-white to-slate-50 p-4 shadow-sm transition-all duration-200 ease-out',
	Interactive: 'hover:-translate-y-0.5 hover:shadow-md',
	Title: 'text-base font-semibold tracking-tight text-slate-900',
	Content: 'text-sm leading-5 text-slate-600 whitespace-pre-line',
	Placeholder:
		'flex h-full w-full items-center justify-center rounded-2xl border-2 border-dashed border-amber-400 bg-amber-100/80',
	PlaceholderText: 'block w-full text-center text-sm font-medium text-amber-800',
} as const
