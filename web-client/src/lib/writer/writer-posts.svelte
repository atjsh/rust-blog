<script lang="ts">
	export let posts: Array<{
		id: number;
		title: string;
	}>;
</script>

<ul>
	{#each posts as post}
		<li>
			<a href={`/post/${post.id}`}>{post.title}</a>
		</li>
	{/each}
</ul>
