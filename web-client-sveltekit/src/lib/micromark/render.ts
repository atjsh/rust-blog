import { micromark } from 'micromark';
import { directive, directiveHtml } from 'micromark-extension-directive';
import { gfm, gfmHtml } from 'micromark-extension-gfm';
import { imgTagHandler } from './tag-handlers';

export function getHTMLFromMarkdown(markdown: string): string {
	return micromark(markdown, {
		allowDangerousHtml: true,
		extensions: [directive(), gfm()],
		htmlExtensions: [directiveHtml({ img: imgTagHandler }), gfmHtml()]
	});
}
