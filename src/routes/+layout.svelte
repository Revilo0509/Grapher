<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import { ModeWatcher } from 'mode-watcher';
	import { initWasm } from '$lib/wasm.svelte';
	import { onMount } from 'svelte';

	let { children } = $props();

	onMount(async () => {
		await initWasm();
	});
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<ModeWatcher />
<div class="flex h-dvh w-dvw">
	{@render children()}
</div>
