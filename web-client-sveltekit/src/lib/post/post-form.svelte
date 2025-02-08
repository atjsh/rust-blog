<script lang="ts">
	import { html } from '@codemirror/lang-html';
	import { markdown } from '@codemirror/lang-markdown';
	import { micromark } from 'micromark';
	import CodeMirror from 'svelte-codemirror-editor';
	import type { PostContentType } from '../api';
	import { getContentTypeLabel } from './post-utils';

	const FileUploadStatus = {
		PENDING: 'pending',
		UPLOADING: 'uploading',
		UPLOADED: 'uploaded',
		FAILED: 'failed'
	} as const;

	type FileUploadUploaded = {
		id: string;
		url: string;
		file: File;
		success: typeof FileUploadStatus.UPLOADED;
	};
	type FileUploadFailed = {
		id: string;
		file: File;
		failedReason: string;
		success: typeof FileUploadStatus.FAILED;
	};
	type FileUploadPending = { id: string; file: File; success: typeof FileUploadStatus.PENDING };
	type FileUploadUploading = { id: string; file: File; success: typeof FileUploadStatus.UPLOADING };

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

	let fileUploadsQueue: (
		| FileUploadPending
		| FileUploadUploading
		| FileUploadUploaded
		| FileUploadFailed
	)[] = [];

	$: renderedContent =
		postValues.contentType == 'md' ? micromark(postValues.content) : postValues.content;

	async function createPostAttachment(file: File) {
		const formData = new FormData();
		formData.append('attachment', file);
		formData.append('fileExtension', file.name.split('.').pop() ?? '');

		const result = await fetch('/post-attachments', {
			method: 'POST',
			body: formData,
			credentials: 'include'
		});

		if (!result.ok) {
			const resultJson: { message: string } = await result.json();
			throw new Error(resultJson.message);
		}

		const resultJson: { id: string; url: string } = await result.json();
		return resultJson;
	}
</script>

<div class="formbox">
	<div class="form-section">
		<h2>
			<label for="attachment">서버에 파일 등록하기</label>
		</h2>
		<p>
			파일을 등록하면, 해당 파일의 URL이 생성됩니다. 이 URL을 이용하여 이미지나 다른 미디어를 내용에
			삽입할 수 있습니다.
		</p>

		<!-- carousel of file upload queue. 
		 it can have remove button, and shows uploading status by green/yellow/red dot. 
		 if uploaded, the url is copied to clipboard. 
		 there are plus button on left. 
		 when plus button get clicked, it shows file selection window. 
		 after user select file, the selected item will added to right side of plus button.
		 file should be uploaded one by one when they added to queue. when user successfully upload file, user can upload another file. -->
		<div class="file-upload-queue">
			<input
				type="file"
				name="attachment"
				id="attachment"
				on:change={(e) => {
					const files = (e.target as HTMLInputElement).files;
					if (!files) return;

					for (let i = 0; i < files.length; i++) {
						const file = files[i];
						const id = Math.random().toString(36).substring(7);
						fileUploadsQueue.push({ id, file, success: FileUploadStatus.PENDING });
					}

					for (let i = 0; i < fileUploadsQueue.length; i++) {
						const fileUpload = fileUploadsQueue[i];
						if (fileUpload.success != FileUploadStatus.PENDING) continue;

						fileUploadsQueue[i] = { ...fileUpload, success: FileUploadStatus.UPLOADING };
						fileUploadsQueue = fileUploadsQueue;
						createPostAttachment(fileUpload.file)
							.then((result) => {
								fileUploadsQueue[i] = {
									...fileUpload,
									success: FileUploadStatus.UPLOADED,
									url: result.url
								};
								fileUploadsQueue = fileUploadsQueue;
							})
							.catch((error) => {
								fileUploadsQueue[i] = {
									...fileUpload,
									success: FileUploadStatus.FAILED,
									failedReason: error.message
								};
								fileUploadsQueue = fileUploadsQueue;
							});
					}
					(e.target as HTMLInputElement).value = '';
				}}
			/>
			{#each fileUploadsQueue as fileUpload}
				<div class="file-upload-item">
					{#if fileUpload.success == FileUploadStatus.PENDING}
						<div class="file-upload-item-pending">
							<span class="file-emoji">🕒</span>
							{fileUpload.file.name}
							<button
								on:click={() => {
									fileUploadsQueue = fileUploadsQueue.filter((file) => file.id != fileUpload.id);
								}}
							>
								취소
							</button>
						</div>
					{:else if fileUpload.success == FileUploadStatus.UPLOADING}
						<div class="file-upload-item-uploading">
							<span class="file-emoji">⏳</span>
							{fileUpload.file.name}
							<div class="uploading-status"></div>
						</div>
					{:else if fileUpload.success == FileUploadStatus.UPLOADED}
						<div class="file-upload-item-uploaded">
							{#if fileUpload.file.type.startsWith('image/')}
								<img src={fileUpload.url} alt="thumbnail" class="thumbnail" />
							{:else}
								<span class="file-emoji">✅</span>
							{/if}
							{fileUpload.file.name}
							<button
								on:click={() => {
									navigator.clipboard.writeText(fileUpload.url);
								}}
							>
								복사
							</button>
						</div>
					{:else if fileUpload.success == FileUploadStatus.FAILED}
						<div class="file-upload-item-failed">
							<span class="file-emoji">❌</span>
							{fileUpload.file.name}
							<button
								on:click={() => {
									fileUploadsQueue = fileUploadsQueue.filter((file) => file.id != fileUpload.id);
								}}
							>
								취소
							</button>
						</div>
					{/if}
				</div>
			{/each}
		</div>
	</div>
</div>
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
				lineWrapping={true}
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
	form,
	.formbox {
		margin: 0 auto;
		width: 100%;
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
			box-sizing: border-box;
			font-size: 1em;
			padding: 0.5em;
			border-style: solid;
			border-radius: 0.4em;
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

			@media (max-width: 1000px) {
				flex-direction: column;
				flex-flow: column-reverse;
			}

			@media (prefers-color-scheme: dark) {
				background: white;
				color: black;
			}

			:global(.content-editor-item) {
				max-width: 50%;
				display: inline-block;
				box-sizing: border-box;

				@media (max-width: 1000px) {
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

		.file-upload-item-pending,
		.file-upload-item-uploading,
		.file-upload-item-uploaded,
		.file-upload-item-failed {
			display: flex;
			align-items: center;
			gap: 0.5em;
		}

		.file-upload-item-uploaded {
			flex-direction: column;
			align-items: flex-start;

			.thumbnail {
				margin-top: 0.5em;
				max-width: 100px;
				max-height: 100px;
				border: 1px solid #d9d9d9;
				border-radius: 0.5em;
			}

			.file-emoji {
				font-size: 2em;
			}
		}
	}
</style>
