import { PostAd, type PostContentType } from '../api';

export function getContentTypeLabel(contentType: PostContentType) {
	switch (contentType) {
		case 'html':
			return 'HTML';
		case 'md':
			return 'Markdown';
	}
}

export function getPostAdLabel(ad: PostAd) {
	switch (ad) {
		case PostAd.NoAd:
			return '미게시';
		case PostAd.CoupangAd:
			return 'Coupang Ad';
	}
}
