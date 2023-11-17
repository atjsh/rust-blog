<script lang="ts">
	import { html } from '@codemirror/lang-html';
	import { markdown } from '@codemirror/lang-markdown';
	import { micromark } from 'micromark';
	import CodeMirror from 'svelte-codemirror-editor';
	import type { PostContentType } from '../api';
	import { getContentTypeLabel } from './post-utils';

	export let errorMessage: string | undefined;
	export let availableCategories: { id: number; name: string }[];

	export let defaultPostValues: {
		categoryId?: number;
		title?: string;
		private?: boolean;
		content?: string;
		contentType?: PostContentType;
	} = {};

	let postValues = {
		categoryId: defaultPostValues.categoryId ?? availableCategories[0]?.id,
		title: defaultPostValues.title ?? '',
		private: defaultPostValues.private ?? true,
		content: defaultPostValues.content ?? '',
		contentType: defaultPostValues.contentType ?? 'html'
	};

	$: renderedContent =
		postValues.contentType == 'md' ? micromark(postValues.content) : postValues.content;
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
			placeholder="제목을 입력하세요..."
			bind:value={postValues.title}
		/>
	</div>

	<div class="form-section">
		<h2>
			<label for="contentType">컨텐츠 타입</label>
		</h2>

		<select name="contentType" id="contentType" bind:value={postValues.contentType}>
			<option value="html">{getContentTypeLabel('html')}</option>
			<option value="md">{getContentTypeLabel('md')}</option>
		</select>
	</div>

	<div class="form-section">
		<h2>
			<label for="content">내용</label>
		</h2>

		<div class="content-editor-container">
			<CodeMirror
				class="content-editor content-editor-item"
				placeholder="{postValues.contentType &&
					getContentTypeLabel(postValues.contentType)} 양식으로 내용을 입력하세요..."
				bind:value={postValues.content}
				lang={postValues.contentType == 'md' ? markdown() : html()}
			/>
			<input type="hidden" name="content" value={postValues.content} />
			<div class="html-preview content-editor-item">
				{@html renderedContent}
			</div>
		</div>
	</div>

	<div class="form-section">
		<h2>
			<label for="isPrivate">비공개/공개 여부</label>
		</h2>

		<input type="checkbox" name="isPrivate" id="isPrivate" bind:checked={postValues.private} />
		<label for="isPrivate">게시글을 비공개 상태로 저장</label>
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
		margin: 0 auto;
		padding: 1em;
		display: flex;
		flex-direction: column;
		gap: 2em;
		box-sizing: border-box;

		select {
			font-size: 1em;
			padding: 0.3em;
		}

		input {
			width: 100%;
			font-size: 1em;
			padding: 0.5em;
			border-style: solid;
			border-radius: 0.4em;

			@media (prefers-color-scheme: dark) {
				background: rgb(var(--theme-bg-color));
				border-color: rgba(var(--theme-text-color), 0.3);
				color: rgb(var(--theme-text-color));
			}
		}

		input[type='checkbox'] {
			width: unset;
		}

		.content-editor-container {
			display: flex;
			gap: 1em;
			height: 30rem;
			position: relative;
			width: 100%;

			@media (max-width: 768px) {
				flex-direction: column;
				flex-flow: column-reverse;
			}

			:global(.content-editor-item) {
				max-width: 50%;
				display: inline-block;
				box-sizing: border-box;

				@media (max-width: 768px) {
					max-width: unset;
					width: 100%;
				}
			}

			:global(.content-editor) {
				flex: 1;
				border: unset;
				border-radius: 0.5em;
				resize: none;
				border: 1px solid #d9d9d9;
				padding: 1em;
				overflow: auto;

				@media (prefers-color-scheme: dark) {
					background: black;
					color: white;
					border: 1px solid #303030;
				}
			}

			.html-preview {
				flex: 1;
				border: unset;
				border-radius: 0.5em;
				padding: 0.3em;
				background: #f5f5f5;
				overflow: auto;
				padding: 1em;

				@media (prefers-color-scheme: dark) {
					background: #303030;
				}
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
