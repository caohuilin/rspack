const path = require("path");
const { describeByWalk, createHotCase } = require("../dist");

describeByWalk(
	__filename,
	(name, src, dist) => {
		createHotCase(name, src, dist, "web");
	},
	{
		source: path.resolve(__dirname, "./hotCases"),
		dist: path.resolve(__dirname, `./js/hot-web`)
	}
);
