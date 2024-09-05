<script>
	import { GeolocateControl, MapLibre } from 'svelte-maplibre';
	import '../styles.scss';
</script>

<main>
	<MapLibre
		style="https://basemaps.cartocdn.com/gl/positron-gl-style/style.json"
		class="maplibre relative w-full sm:aspect-video h-screen z-0"
		zoom={15}
		center={[20.98834, 52.23666]}
	>
		<GeolocateControl
			position="top-left"
			fitBoundsOptions={{ maxZoom: 12 }}
			showAccuracyCircle={false}
		/>
	</MapLibre>

	<!-- <slot /> -->
</main>
