local M = {}

local function read_file(path)
	local file = io.open(path, 'rb')
	if not file then return nil end
	local content = file:read '*a'
	file:close()
	return content
end

function M.command(cmd)
	local stdout_path = os.tmpname()
	cmd = cmd .. ' 1>' .. stdout_path

	local stderr_path = os.tmpname()
	cmd = cmd .. ' 2>' .. stderr_path

	local process = io.popen(cmd, 'r')
	local _, _, exitcode = process:close()

	local stdout = read_file(stdout_path)
	os.remove(stdout_path)

	local stderr = read_file(stderr_path)
	os.remove(stderr_path)

	return exitcode, stdout, stderr
end

return M
