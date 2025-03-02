import type { Handle } from 'micromark-extension-directive';

export const imgTagHandler: Handle = function (this, d) {
	if (d.type !== 'textDirective' && d.type !== 'leafDirective') {
		return false;
	}

	this.tag('<img');

	if (d.label) {
		this.tag(' alt="');
		this.raw(this.encode(d.label));
		this.tag('"');
	} else {
		this.tag(' alt=""');
	}

	if (d.attributes && d.attributes.src) {
		this.tag(' src="');
		this.raw(this.encode(String(d.attributes.src)));
		this.tag('"');
	} else {
		this.tag(' src=""');
	}

	if (d.attributes && d.attributes.class) {
		this.tag(' class="');
		this.raw(this.encode(String(d.attributes.class)));
		this.tag('"');
	}

	this.tag('>');

	return true;
};
