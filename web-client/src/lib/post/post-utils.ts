import type { PostContentType } from '../api';

export function getContentTypeLabel(contentType: PostContentType) {
	switch (contentType) {
		case 'html':
			return 'HTML';
		case 'md':
			return 'Markdown';
	}
}
