function PLUGIN:BackendPreInstall(ctx)
	return {
		url = "https://example.com/"
			.. ctx.tool
			.. "/"
			.. ctx.version
			.. "/"
			.. RUNTIME.osType
			.. "-"
			.. RUNTIME.archType
			.. ".tar.gz",
		sha256 = "dummychecksum_" .. ctx.tool .. "_" .. ctx.version .. "_" .. RUNTIME.osType .. "_" .. RUNTIME.archType,
		size = 12345678,
	}
end
