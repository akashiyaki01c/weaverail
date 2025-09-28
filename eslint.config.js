import { FlatCompat } from '@eslint/eslintrc';
import reactHooks from 'eslint-plugin-react-hooks';
import reactRefresh from 'eslint-plugin-react-refresh';
import importPlugin from 'eslint-plugin-import';
import tseslint from '@typescript-eslint/eslint-plugin';

const compat = new FlatCompat();

export default [
	{
		ignores: ['dist/**', 'src-tauri/**'],
	},
	{
		files: ['**/*.{js,jsx,ts,tsx}'],
		languageOptions: {
			ecmaVersion: 2020,
			parser: '@typescript-eslint/parser',
		},
		plugins: {
			'react-hooks': reactHooks,
			'react-refresh': reactRefresh,
			import: importPlugin,
			'@typescript-eslint': tseslint,
		},
		settings: {
			react: {
				version: 'detect',
			},
			'import/resolver': {
				typescript: {},
			},
		},
		rules: {
			...reactHooks.configs.recommended.rules,
			...tseslint.configs.recommended.rules,
		},
	},
	...compat.extends(
		'plugin:@typescript-eslint/recommended',
		'plugin:import/recommended',
		'prettier'
	),
];
