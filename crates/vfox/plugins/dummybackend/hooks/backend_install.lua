function PLUGIN:BackendInstall(ctx)
	local tool = ctx.tool
	local version = ctx.version
	local install_path = ctx.install_path

	-- Create bin directory
	os.execute("mkdir -p " .. install_path .. "/bin")

	-- Create a dummy executable that prints the version
	local bin_path = install_path .. "/bin/" .. tool
	local f = io.open(bin_path, "w")
	if f then
		f:write("#!/bin/sh\n")
		f:write('echo "' .. version .. '"\n')
		f:close()
	end
	os.execute("chmod +x " .. bin_path)

	return {}
end
