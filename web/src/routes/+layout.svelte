<script lang="ts">
	import '../app.css';
	import { onMount } from 'svelte';
	import Menu from './Menu.svelte';
	import { routes } from './routes';
	import Lightswitch from '$lib/components/Lightswitch.svelte';
	import { auth } from '$lib/auth.svelte';
	import { page } from '$app/state';
	import { dates, setClasses } from '$lib/dates';

	let { children } = $props();

	let hideMenu = $state(false);

	onMount(async () => {
		setClasses();
	});

	const breadcrumb = $derived.by(() => {
		const route = routes.find((r) => page.url.pathname.startsWith(r.path));
		if (!route) {
			return [];
		}
		const pieces = page.url.pathname
			.substring(route.path.length + 1)
			.split('/')
			.filter((r) => r)
			.map((r) => decodeURIComponent(r));
		pieces.reverse();
		pieces.push(decodeURIComponent(route.name));
		pieces.reverse();
		return pieces;
	});
</script>

<header class="mx-auto flex h-16 w-full shrink-0 items-center gap-2 bg-gray-800 pr-8 pl-4">
	<button
		aria-label="menu"
		onclick={() => (hideMenu = !hideMenu)}
		class="rounded-md p-2 leading-[0] hover:bg-gray-700"
	>
		<span class="icon-[material-symbols--menu-rounded] bg-gray-200 text-[24px]"></span>
	</button>
	<div class="flex flex-1 items-center gap-2 text-white">
		<p class="logo mr-2 text-lg">logistics</p>
		{#each breadcrumb as piece, i (i)}
			<p>{piece}</p>
			{#if i + 1 != breadcrumb.length}
				<p>&gt;</p>
			{/if}
		{/each}
	</div>
	<Lightswitch></Lightswitch>

	{#await auth}
		<span></span>
	{:then auth}
		<img class="max-h-full p-2 rounded-md" src={auth.picture} alt="" />
		<div class="flex flex-col text-xs text-white">
			<span>{auth.name}</span>
			<span>{auth.level}</span>
		</div>
	{/await}
</header>

<nav class="relative flex flex-1">
	<main
		class:ml-0={hideMenu}
		class:ml-64={!hideMenu}
		class="ml-64 flex-1 transition-[margin-left] duration-300 ease-in-out"
	>
		{@render children()}
	</main>
	<nav
		class:-left-64={hideMenu}
		class:left-0={!hideMenu}
		class="absolute top-0 z-10 flex h-full w-64 transform flex-col justify-between bg-gray-100 transition-[left] duration-300 ease-in-out dark:bg-gray-700"
	>
		<Menu></Menu>
	</nav>
</nav>

{#if dates.xmas}
	<!-- https://codepen.io/alphardex/pen/dyPorwJ -->
	<section class="fixed z-11">
		{#each { length: 200 } as _ (_)}
			<div class="snow"></div>
		{/each}
	</section>
{/if}

<style lang="scss">
	@use 'sass:math';
	:global(html, body) {
		margin: 0;
		padding: 0;
		height: 100vh;
	}
	:global(html[data-date='xmas'] .snow) {
		@function random_range($min, $max) {
			$rand: math.random();
			$random_range: $min + math.floor($rand * (($max - $min) + 1));
			@return $random_range;
		}

		$total: 200;
		position: absolute;
		width: 10px;
		height: 10px;
		background: white;
		border-radius: 50%;

		@for $i from 1 through $total {
			$random-x: math.random(1000000) * 0.0001vw;
			$random-offset: random_range(-100000, 100000) * 0.0001vw;
			$random-x-end: $random-x + $random-offset;
			$random-x-end-yoyo: $random-x + calc($random-offset / 2);
			$random-yoyo-time: calc(random_range(30000, 80000) / 100000);
			$random-yoyo-y: $random-yoyo-time * 100vh;
			$random-scale: math.random(10000) * 0.0001;
			$fall-duration: random_range(10, 30) * 1s;
			$fall-delay: math.random(30) * -1s;

			&:nth-child(#{$i}) {
				opacity: math.random(10000) * 0.0001;
				transform: translate($random-x, -10px) scale($random-scale);
				animation: fall-#{$i} $fall-duration $fall-delay linear infinite;
			}

			@keyframes fall-#{$i} {
				#{math.percentage($random-yoyo-time)} {
					transform: translate($random-x-end, $random-yoyo-y) scale($random-scale);
				}

				to {
					transform: translate($random-x-end-yoyo, 100vh) scale($random-scale);
				}
			}
		}
	}
	:global(html[data-date='pride'] .logo span) {
		&:nth-child(1) {
			color: red;
		}
		&:nth-child(2) {
			color: orange;
		}
		&:nth-child(3) {
			color: yellow;
		}
		&:nth-child(4) {
			color: green;
		}
		&:nth-child(5) {
			color: blue;
		}
		&:nth-child(6) {
			color: purple;
		}
	}
</style>
