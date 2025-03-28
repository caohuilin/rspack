it("should watch for changes", function () {
	if (+WATCH_STEP !== 3) expect(require("./delayed")).toBe(WATCH_STEP);
	else expect(require("./delayed")).toBe("This is only a test." + WATCH_STEP);
	if (+WATCH_STEP > 0) {
		for (var m of __STATS__.modules.filter(m =>
			/(a|b|c)\.js$/.test(m.identifier)
		))
			expect(m.issuer).toBe(null);
	}
});
