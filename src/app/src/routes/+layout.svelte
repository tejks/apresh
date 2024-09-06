<script>
	import { defaultLocation } from '$lib/common';
	import { MapLibre } from 'svelte-maplibre';
	import Header from '../components/common/Header.svelte';
	import '../styles.scss';
</script>

<main>
	<Header />

	<MapLibre
		style="https://basemaps.cartocdn.com/gl/positron-gl-style/style.json"
		class="maplibre relative w-full sm:aspect-video h-screen z-0"
		zoom={15}
		center={defaultLocation}
	>
		<slot />
	</MapLibre>
</main>
