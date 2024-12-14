import daisyui from 'daisyui'
import themes from 'daisyui/src/theming/themes'

/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {},
	},
	plugins: [daisyui],
	daisyui: {
		themes: [
			{
				emerald: {
					...themes.emerald,
					primary: '#42b983',
					'primary-content': '#ffffff',
					'--btn-text-case': 'none',
				},
			},
		],
	},
}
