<script lang="ts">
	export let writer: {
		email: string;
		description: string;
		id: number;
	};
</script>

<p>email: {writer.email}</p>
<p>{writer.description}</p>
