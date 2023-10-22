<script lang="ts">
	import type { ActionData, PageData } from './$types';

	export let data: PageData;

	export let form: ActionData;
</script>

<h1>New Post</h1>

<form method="post">
	<label for="categoryId">Category: </label>
	<select name="categoryId" id="categoryId" value={data.post.category.id}>
		{#each data.categories as category}
			<option value={category.id}>{category.name}</option>
		{/each}
	</select>

	<br />

	<input
		type="text"
		name="title"
		id="title"
		required
		placeholder="enter title"
		value={data.post.title}
	/>

	<br />

	<textarea
		name="content"
		id="content"
		cols="30"
		rows="10"
		required
		placeholder="enter your content (HTML) ..."
		value={data.post.content}
	/>

	<br />

	<button type="submit">Post</button>

	{#if form?.error}
		<p style="color: red">{form.error}</p>
	{/if}
</form>
