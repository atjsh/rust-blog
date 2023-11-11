<script lang="ts">
	export let errorMessage: string | undefined;
	export let availableCategories: { id: number; name: string }[];

	export let defaultPostValues: {
		categoryId?: number;
		title?: string;
		private?: boolean;
		content?: string;
	} = {
		categoryId: availableCategories[0]?.id,
		title: '',
		private: true,
		content: ''
	};

	let postValues = defaultPostValues;
</script>

<form method="post">
	<div class="form-section">
		<h2>
			<label for="categoryId">카테고리</label>
		</h2>
		<select name="categoryId" id="categoryId" bind:value={postValues.categoryId}>
			{#each availableCategories as category}
				<option value={category.id}>{category.name}</option>
			{/each}
		</select>
	</div>

	<div class="form-section">
		<h2>
			<label for="title">제목</label>
		</h2>

		<input
			type="text"
			name="title"
			id="title"
			required
			placeholder="enter title"
			bind:value={postValues.title}
		/>
	</div>

	<div class="form-section">
		<h2>
			<label for="content">내용</label>
		</h2>

		<div class="html-editor-container">
			<textarea
				class="html-editor"
				name="content"
				id="content"
				cols="30"
				rows="10"
				required
				placeholder="enter your content (HTML) ..."
				bind:value={postValues.content}
			/>
			<div class="html-preview">
				{@html postValues.content}
			</div>
		</div>
	</div>

	<div class="form-section">
		<h2>
			<label for="private">비공개/공개 여부</label>
		</h2>

		<input type="checkbox" name="private" id="private" bind:checked={postValues.private} />
		{#if postValues.private}
			<label for="private">게시글을 비공개 상태로 저장</label>
		{:else}
			<label for="private">게시글을 공개 상태로 저장</label>
		{/if}

		<div class="buttons">
			<button type="submit">Post</button>
		</div>

		{#if errorMessage}
			<p style="color: red">{errorMessage}</p>
		{/if}
	</div>
</form>

<style lang="scss">
	form {
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

		input[type='checkbox'] {
			width: unset;
		}

		textarea {
			padding: 0.3em;
		}

		.html-editor-container {
			display: flex;
			gap: 1em;
			height: 30rem;

			.html-editor {
				flex: 1;
				border: unset;
				border-radius: 0.5em;
				resize: none;
				border: 1px solid #d9d9d9;
				padding: 1em;
			}

			.html-preview {
				flex: 1;
				border: unset;
				border-radius: 0.5em;
				padding: 0.3em;
				background: #f5f5f5;
				overflow: auto;
				padding: 1em;
			}
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
