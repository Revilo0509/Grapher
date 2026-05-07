<script lang="ts">
    import { onMount } from 'svelte';

    let canvas = $state<HTMLCanvasElement>();
    let context = $state<CanvasRenderingContext2D>();

    onMount(() => {
        if (canvas) {
            context = canvas.getContext('2d')!;
        }
    });
</script>

<canvas bind:this={canvas} class="w-full h-full"></canvas>
