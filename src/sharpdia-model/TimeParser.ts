export function parseTime(text: string) {
	switch (text.length) {
		case 3: {
			const hour = Number.parseInt(text.substring(0, 1));
			const minute = Number.parseInt(text.substring(1, 3));
			return hour * 3600 + minute * 60;
		}
		case 4: {
			const hour = Number.parseInt(text.substring(0, 2));
			const minute = Number.parseInt(text.substring(2, 4));
			return hour * 3600 + minute * 60;
		}
		case 5: {
			const hour = Number.parseInt(text.substring(0, 1));
			const minute = Number.parseInt(text.substring(1, 3));
			const second = Number.parseInt(text.substring(3, 5));
			return hour * 3600 + minute * 60 + second;
		}
		case 6: {
			const hour = Number.parseInt(text.substring(0, 2));
			const minute = Number.parseInt(text.substring(2, 4));
			const second = Number.parseInt(text.substring(4, 6));
			return hour * 3600 + minute * 60 + second;
		}
		default: 
			return Number.parseInt(text);
	}
}

export function toTimeString(time: number) {
	const hour = Math.floor(time / 3600);
	const minute = Math.floor(time / 60) % 60;
	const second = time % 60;

	return `${hour}:${minute.toString().padStart(2, "0")}:${second.toString().padStart(2, "0")}`;
}