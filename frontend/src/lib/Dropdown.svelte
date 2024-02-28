<!-- adapted from https://svelte.dev/examples/select-bindings -->
<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import "../app.css";

    interface Option {
        id: Number;
        text: String;
    }

    const dispatch = createEventDispatcher();


	export let options: Option[] = [];
	let selected: Option | undefined = options[0];
    // https://svelte.dev/examples/component-events
    function exportSelected() {
        if (selected !== undefined) {
            dispatch("message", selected);
        }
    }
    
</script>

<style>
	select {
        background-color: var(--primary-bg-color);
        color: var(--primary-text-color);
        margin-left: 1%;
        margin-top: 1%;
        text-align: center;
        font-size: larger;
        border: var(--primary-bg-color);
	}
</style>

<!-- <form on:submit|preventDefault={handleSubmit}> -->
	<select bind:value={selected} on:change={exportSelected}>
		{#each options as option}
			<option value={option}>
				{option.text}
			</option>
		{/each}
	</select>
<!-- </form> -->

<!-- <p>selected question {selected ? selected.id : '[waiting...]'}</p> -->