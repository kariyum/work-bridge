// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		interface PageState {
			projectEditMode: boolean = false;
			showTaskPopup: boolean = false;
		}
		// interface Platform {}
	}
}

export {};
