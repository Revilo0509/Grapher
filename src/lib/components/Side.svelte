<script module lang="ts">
    export class Equation {
        id: string;
        content = $state("");

        constructor(id: string, content = "") {
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

<script>
	import { SvelteDate, SvelteSet } from "svelte/reactivity";
	import Input from "./ui/input/input.svelte";
	import Button from "./ui/button/button.svelte";
	import { X } from "lucide-svelte";
</script>
<div class="bg-accent w-64 flex flex-col p-2">
    <div class="flex-1 overflow-auto">
        {#each equations as equation (equation.id)}
            <div class="flex mb-2">
                <Input bind:value={equation.content} class="mr-2"/>
                <Button onclick={() => removeEquation(equation.id)}><X /></Button>
            </div>            
        {/each}
    </div>
    <Button onclick={() => {equations.add(new Equation(SvelteDate.now().toString()))}}>Add</Button>
</div>