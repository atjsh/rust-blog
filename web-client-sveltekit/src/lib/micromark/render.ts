import { micromark } from 'micromark';
import { directive, directiveHtml } from 'micromark-extension-directive';
import { imgTagHandler } from './tag-handlers';

export function getHTMLFromMarkdown(markdown: string): string {
	return micromark(markdown, {
		extensions: [directive()],
		htmlExtensions: [directiveHtml({ img: imgTagHandler })]
	});
}
