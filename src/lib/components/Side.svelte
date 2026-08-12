<script module lang="ts">
	import { SvelteDate, SvelteSet } from 'svelte/reactivity';
	import Input from './ui/input/input.svelte';
	import Button from './ui/button/button.svelte';
	import { X } from 'lucide-svelte';

	export class Equation {
		id: string;
		content = $state('');

		constructor(id: string, content = '') {
			this.id = id;
			this.content = content;
		}
	}

	export let equations = new SvelteSet<Equation>();

	export function removeEquation(id: string) {
		for (const equation of equations) {
			if (equation.id === id) {
				equations.delete(equation);
				break;
			}
		}
	}
</script>

<div class="flex w-64 flex-col bg-accent p-2">
	<div class="no-scrollbar flex-1 overflow-auto">
		{#each equations as equation (equation.id)}
			<div class="mb-2 flex">
				<Input bind:value={equation.content} class="mr-2" placeholder="Enter your equation..." />
				<Button onclick={() => removeEquation(equation.id)}><X /></Button>
			</div>
		{/each}
	</div>
	<Button
		onclick={() => {
			equations.add(new Equation(SvelteDate.now().toString()));
		}}
	>
		Add
	</Button>
</div>
