const today = new Date();

const month = today.getUTCMonth() + 1;
const day = today.getUTCDate();
// const weekday = today.getUTCDay() || 7;

export const dates = {
	xmas: month == 12 && day > 10,
	pride: month == 6
};

export const setClasses = () => {
	const match = Object.entries(dates).find(([_, b]) => b);
	if (match) {
		document.documentElement.setAttribute('data-date', match[0]);
	}
};
