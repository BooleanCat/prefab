local M = {}

function M.execute(command)
	local cmd, stdout_path, stderr_path = make_command(command)
	local handle = io.popen(cmd, 'r')
	local exitstatus = {handle:close()}
	stdout = read_file(stdout_path)
	stderr = read_file(stderr_path)
	os.remove(stderr_path)
	os.remove(stdout_path)
	return {
		exitcode=exitstatus[3],
		stdout=stdout,
		stderr=stderr,
	}
end

function read_file(path)
	local file = io.open(path, 'rb')
	if not file then return nil end
	local content = file:read '*a'
	file:close()
	return content
end

function make_command(cmd)
	local stdout_path = os.tmpname()
	local stderr_path = os.tmpname()
	return cmd .. ' 1>' .. stdout_path .. ' 2>' .. stderr_path, stdout_path, stderr_path
end

return M
