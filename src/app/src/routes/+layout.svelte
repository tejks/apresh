<script>
	import { GeolocateControl, MapLibre } from 'svelte-maplibre';
	import '../styles.scss';
	import { defaultLocation } from '$lib/common';
</script>

<main>
	<MapLibre
		style="https://basemaps.cartocdn.com/gl/positron-gl-style/style.json"
		class="maplibre relative w-full sm:aspect-video h-screen z-0"
		zoom={15}
		center={defaultLocation}
	>
		<GeolocateControl
			position="top-left"
			fitBoundsOptions={{ maxZoom: 12 }}
			showAccuracyCircle={false}
		/>

		<slot />
	</MapLibre>
</main>

<style lang="scss">
</style>
