<script lang="ts">
	import { onMount } from 'svelte';
	import { equations } from './Side.svelte';
	import { isInitialized } from '$lib/wasm.svelte';
	import { draw } from 'rust';

	let canvas = $state<HTMLCanvasElement>();
	let context = $state<CanvasRenderingContext2D>();

	let width = $state<number>();
	let height = $state<number>();
	let pixelWidth = $state(0);
	let pixelHeight = $state(0);

	function resizeCanvas() {
		if (!canvas) return;
		const dpr = window.devicePixelRatio || 1;
		const w = canvas.clientWidth * dpr;
		const h = canvas.clientHeight * dpr;
		canvas.width = w;
		canvas.height = h;
		pixelWidth = w;
		pixelHeight = h;
		if (context) {
			context.setTransform(dpr, 0, 0, dpr, 0, 0);
		}
	}

	$effect(() => {
		if (!isInitialized() || !context || !pixelWidth || !pixelHeight) return;

		let equ: string[] = [];
		equations.forEach((i) => {
			equ.push(i.content);
		});

		draw(context, pixelWidth, pixelHeight, equ);
	});

	onMount(() => {
		if (canvas) {
			context = canvas.getContext('2d')!;
			resizeCanvas();

			const ro = new ResizeObserver(() => {
				resizeCanvas();
			});
			ro.observe(canvas);

			return () => ro.disconnect();
		}
	});
</script>

<canvas bind:this={canvas} class="h-full w-full"> </canvas>

<style>
	canvas {
		image-rendering: pixelated;
	}
</style>
