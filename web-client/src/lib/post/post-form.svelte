<script lang="ts">
	export let errorMessage: string | undefined;
	export let availableCategories: { id: number; name: string }[];

	export let defaultPostValues: {
		categoryId?: number;
		title?: string;
		content?: string;
	} = {
		categoryId: availableCategories[0]?.id,
		title: '',
		content: ''
	};
</script>

<form method="post">
	<div class="form-section">
		<h2>
			<label for="categoryId">Category</label>
		</h2>
		<select name="categoryId" id="categoryId" value={defaultPostValues.categoryId}>
			{#each availableCategories as category}
				<option value={category.id}>{category.name}</option>
			{/each}
		</select>
	</div>

	<div class="form-section">
		<h2>
			<label for="title">Title</label>
		</h2>

		<input
			type="text"
			name="title"
			id="title"
			required
			placeholder="enter title"
			value={defaultPostValues.title}
		/>
	</div>

	<div class="form-section">
		<h2>
			<label for="content">Content</label>
		</h2>

		<textarea
			name="content"
			id="content"
			cols="30"
			rows="10"
			required
			placeholder="enter your content (HTML) ..."
			value={defaultPostValues.content}
		/>
	</div>

	<div class="buttons">
		<button type="submit">Post</button>
	</div>

	{#if errorMessage}
		<p style="color: red">{errorMessage}</p>
	{/if}
</form>

<style lang="scss">
	form {
		max-width: 60rem;
		margin: 0 auto;
		padding: 1em;
		display: flex;
		flex-direction: column;
		gap: 2em;

		select {
			font-size: 1em;
			padding: 0.3em;
		}

		input {
			width: 100%;
			font-size: 1em;
			padding: 0.3em;
		}

		textarea {
			width: 100%;
			padding: 0.3em;
			height: 30rem;
		}

		.buttons {
			display: flex;
			justify-content: right;

			button {
				border: unset;
				border-radius: 10em;
				font-size: 1em;
				padding: 0.8em 1.5em;
				opacity: 1;
				transition: background 0.15s ease;
				cursor: pointer;

				background: #0080ff;
				color: white;

				&:hover {
					background: #4aa4ff;
				}
			}
		}
	}
</style>
