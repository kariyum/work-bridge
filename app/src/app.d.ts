// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		interface PageState {
			projectEditMode: boolean;
			showTaskPopup: boolean;
			profileEditMode: boolean;
		}
		// interface Platform {}
	}
}

export {};
