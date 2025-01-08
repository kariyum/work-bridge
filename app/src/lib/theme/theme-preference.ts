import { BROWSER } from 'esm-env';

export type Theme = 'light' | 'dark'; 

export interface SystemThemePreference { getCurrentSystemPreference: () => Theme; }

export const systemThemePreference: SystemThemePreference = {
	getCurrentSystemPreference: () => {
		if (!BROWSER) return 'dark';
		return window.matchMedia('(prefers-color-scheme: dark)').matches?'dark' :'light';
	}
};

