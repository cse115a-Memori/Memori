import { Sound } from 'svelte-sound'

import alarm_wav from '$lib/assets/alarm.wav'
import retro_wav from '$lib/assets/retro.wav'

// export class
const sound = $state({ isPlaying: false })

const click_sound = new Sound(retro_wav)
const alarm_sound = new Sound(alarm_wav, {
	onend: () => (sound.isPlaying = true),
})

export function playSuccessSound() {
	sound.isPlaying = true
	click_sound.play()
}

export function playFailedSound() {
	sound.isPlaying = true
	alarm_sound.play()
}
