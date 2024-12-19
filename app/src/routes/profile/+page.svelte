<script lang="ts">
	import Skills from "$lib/components/Skills.svelte";
	import { TaskClass } from "$lib/states.svelte";

    let { data } = $props();
    let skills: string[] = $state([]);
    const addSkill = (skill: string) => skills.push(skill);
    const removeSkillAtIndex = (index: number) => skills.splice(index, 1);
    let form: HTMLFormElement;
    async function saveProfile(event: SubmitEvent) {
        event.preventDefault();
        const formData = new FormData(form);
        const entries = formData.entries().filter(([key, value]) => key !== "first_name");
        const profile = Object.fromEntries(entries);
        const payload = {
            ...profile,
            skills,
            birthdate: new Date(profile.birthdate as string).toISOString(),
        }
        fetch("/api/profile", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(payload),
        })
            .then((res) => res.json())
            .then((data) => {
                console.log(data);
            });
    }
</script>

<div class="container">
    <h2>Welcome {data.user?.email}</h2>
    
    <p>Edit your profile</p>
    <form bind:this={form} onsubmit={saveProfile}>
        <label for="description">Tell us about yourself</label>
        <textarea id="description" placeholder="Tell us about yourself" name="description" value={""}></textarea>
        <label for="first_name">First Name</label>
        <input type="text" placeholder="First name" id="first_name" name="first_name" value={"ddd"} />
        <label for="last_name">Last Name</label>
        <input type="text" placeholder="Last name" id="last_name" name="last_name" value={"dd"} />
        <label for="skills">Skills</label>
        <div class="skills">
            <Skills skillsIn={skills} {addSkill} {removeSkillAtIndex}  ></Skills>
        </div>
        <label for="dob">Date of Birth</label>
        <input type="date" id="dob" name="birthdate" value={""} />
        <label for="phone_number">Phone Number</label>
        <input type="tel" name="phone" id="phone_number">
        <input type="submit" value="Save">
    </form>
</div>

<style>
    .skills {
        border: 1px solid #ccc;
        padding: 0.5rem;
        border-radius: 5px;
    }
    label {
        width: fit-content;
        margin-left: auto;
    }
    form {
        display: grid;
        grid-template-columns: auto 1fr;
        max-width: 500px;
        gap: 0.5rem 1rem;
    }
</style>
