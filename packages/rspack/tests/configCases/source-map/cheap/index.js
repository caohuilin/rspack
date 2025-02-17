const fs = require("fs");
const sourceMap = require("source-map");
require("./index.scss");

it("should only map transformed lines if cheap options is used", async () => {
	const source = fs.readFileSync(__dirname + "/main.css.map", "utf-8");
	const map = JSON.parse(source);
	expect(map.sources).toContain("index.scss");
	expect(map.file).toEqual("main.css");
	expect(map.sourcesContent[0]).not.toContain("$backgroundColor");
	const consumer = await new sourceMap.SourceMapConsumer(map);
	consumer.eachMapping(m => {
		expect(m.generatedColumn).toBe(0);
		expect(m.originalColumn).toBe(0);
	});
});
