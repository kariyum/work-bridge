import { storage } from '$lib/storage';
import { BROWSER } from 'esm-env';
export const theme = storage('theme', {
	preference: 'system',
	current: 'dark'
});

theme.subscribe(($theme) => {
	if (!BROWSER) return;
	document.documentElement.classList.remove('light', 'dark');
	document.documentElement.classList.add($theme.current);
});

export const themeStore: any = {
	subscribe: theme.subscribe(($theme) => {
        if (!BROWSER) return;
        document.documentElement.classList.remove('light', 'dark');
        document.documentElement.classList.add($theme.current);
    }),
	set: theme.set,
	get: () => {
		let value: any;
		theme.subscribe((state) => (value = state))();
		return value!;
	}
};