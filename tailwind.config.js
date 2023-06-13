/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {},
	},
	plugins: [require('daisyui')],
	daisyui: {
		themes: [
			{
				emerald: {
					...require('daisyui/src/theming/themes')['[data-theme=emerald]'],
					primary: '#42b983',
					'primary-content': '#ffffff',
					'--btn-text-case': 'none',
				},
			},
		],
	},
}
