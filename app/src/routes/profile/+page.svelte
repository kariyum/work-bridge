<script lang="ts">
	import Skills from "$lib/components/Skills.svelte";
	import { TaskClass } from "$lib/states.svelte";

    let { data } = $props();
    let skills: string[] = $state([]);
    const addSkill = (skill: string) => skills.push(skill);
    const removeSkillAtIndex = (index: number) => skills.splice(index, 1);
</script>

<div class="container">
    <h2>Welcome {data.user?.email}</h2>
    
    <p>Edit your profile</p>
    <form>
        <label for="whoisit">Tell us about yourself</label>
        <textarea id="whoisit" placeholder="Tell us about yourself" value={""}></textarea>
        <label for="first_name">First Name</label>
        <input type="text" placeholder="First name" value={""} />
        <label for="last_name">Last Name</label>
        <input type="text" placeholder="Last name" value={""} />
        <label for="skills">Skills</label>
        <div class="skills">
            <Skills skillsIn={skills} {addSkill} {removeSkillAtIndex}  ></Skills>
        </div>
        <button>Save</button>
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
        margin: auto;
        gap: 0.5rem 1rem;
    }
</style>
