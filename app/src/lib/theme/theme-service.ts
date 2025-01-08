import { onDestroy } from 'svelte';
import { BROWSER } from 'esm-env';
import { themeStore } from '$lib/stores/theme';
import { systemThemePreference } from '$lib/theme/theme-preference';

class ThemeService {
	private static instance: ThemeService;

	private constructor(
		private themeStore: any,
		private systemPreferenceDetector: any
	) {}

	public static getInstance(): ThemeService {
		if (!ThemeService.instance) {
			ThemeService.instance = new ThemeService(themeStore, systemThemePreference);
		}
		return ThemeService.instance;
	}

	toggle() {
        console.log("clicked");
		const { current } = this.themeStore.get();
		const newTheme = current === 'light' ? 'dark' : 'light';
		const systemTheme = this.systemPreferenceDetector.getCurrentSystemPreference();

		const preference = newTheme === systemTheme ? 'system' : 'local';
		this.themeStore.set({ preference, current: newTheme });
	}

    getStore() {
        return this.themeStore.get();
    }

	setupSystemPreferenceListener() {
		if (!BROWSER) return;
        console.log("check");
		const query = window.matchMedia('(prefers-color-scheme: dark)');

		const handleSystemThemeChange = (e: MediaQueryListEvent) => {
			const { preference } = this.themeStore.get();
			if (preference === 'system') {
				this.themeStore.set({
					preference: 'system',
					current: e.matches ? 'dark' : 'light'
				});
			}
		};

		query.addEventListener('change', handleSystemThemeChange);
		onDestroy(() => query.removeEventListener('change', handleSystemThemeChange));
	}
}

export const themeService = ThemeService.getInstance();