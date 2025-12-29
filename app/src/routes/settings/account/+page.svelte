<script lang="ts">
	import { validateObject, type ValidationErrors } from '$lib';
	import { Validator } from '$lib/validator.js';

	let { data } = $props();
	let formElement: HTMLFormElement;
	let formErrors: ValidationErrors<PasswordUpdate> | undefined = $state(undefined);

	interface PasswordUpdate {
		currentPassword: any;
		newPassword: any;
		confirmPassword: any;
	}
	interface FormValidation {
		errors: ValidationErrors<PasswordUpdate>;
		hasErrors: boolean;
	}

	function submit(event: SubmitEvent) {
		event.preventDefault();
		const formData = new FormData(formElement);
		const form: PasswordUpdate = {
			currentPassword: formData.get('current_password'),
			newPassword: formData.get('new_password'),
			confirmPassword: formData.get('confirmPassword')
		};
		const validators = {
			currentPassword: Validator.string('current_password').required().nonEmpty(),
			newPassword: Validator.string('new_password').required().nonEmpty().withMinSize(8),
			confirmPassword: Validator.string('confirm_password')
				.required()
				.equal(formData.get('password')?.toString() ?? '', 'password')
		};
		formErrors = validateObject(form, validators);
		const hasErrors = Object.values(formErrors).some((errors) => errors.length > 0);
		if (hasErrors) {
			return;
		}
	}

	function updatePassword(currentPassword: string, newPassword: string, confirmPassword: string) {}
</script>

<h1>Change Your Password</h1>
<form bind:this={formElement}>
	<div class="input-label">
		<input type="text" placeholder=" " name="current_password" />
		<label for="email">Current Password</label>
	</div>
	{#if formErrors}
		{@render errors(formErrors.currentPassword ?? [])}
	{/if}
	<div class="input-label">
		<input type="text" placeholder=" " name="new_password" />
		<label for="email">New Password</label>
	</div>
	{#if formErrors}
		{@render errors(formErrors.newPassword ?? [])}
	{/if}
	<div class="input-label">
		<input type="text" placeholder=" " name="confirm_password" />
		<label for="email">Confirm New Password</label>
	</div>
	{#if formErrors}
		{@render errors(formErrors.confirmPassword ?? [])}
	{/if}
	<button type="submit">Update</button>
</form>
{#snippet errors(errors: string[])}
	{#if errors.length > 0}
		<div class="error-message">
			{#each errors as err}
				<div class="form-reason">{err}</div>
			{/each}
		</div>
	{/if}
{/snippet}

<style>
    form {
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }
	input {
		max-width: 60%;
		width: 100%;
	}
	@media (width < 600px) {
		input {
			max-width: 100%;
		}
	}
</style>
