when = describe

describe('prefab', function()
	when('invoked with -h', function()
		local state

		before_each(function()
			state = execute('./target/release/prefab -h')
		end)

		it('succeeds', function()
			assert.are.equal(0, state.exitcode)
		end)

		it('prints help', function()
			assert.is_not_nil(string.find(state.stdout, 'USAGE'))
		end)
	end)

	when('invoked with -V', function()
		local state

		before_each(function()
			state = execute('./target/release/prefab -V')
		end)

		it('succeeds', function()
			assert.are.equal(0, state.exitcode)
		end)

		it('prints the version', function()
			assert.are.equal("prefab 0.1.0\n", state.stdout)
		end)
	end)

	when('invoked incorrectly', function()
		local state

		before_each(function()
			state = execute('./target/release/prefab')
		end)

		it('fails', function()
			assert.are_not.equal(0, state.exitcode)
		end)

		it('prints usage to stderr', function()
			assert.is_not_nil(string.find(state.stderr, 'USAGE'))
		end)
	end)

	describe('create', function()
		it('suceeds', function()
			local state = execute('./target/release/prefab create foo')
			assert.are.equal(0, state.exitcode)
		end)

		when('invoked incorrectly', function()
			local state

			before_each(function()
				state = execute('./target/release/prefab create')
			end)

			it('fails', function()
				assert.are_not.equal(0, state.exitcode)
			end)

			it('prints usage to stderr', function()
				assert.is_not_nil(string.find(state.stderr, 'USAGE'))
			end)
		end)
	end)

	describe('state', function()
		it('suceeds', function()
			local state = execute('./target/release/prefab state foo')
			assert.are.equal(0, state.exitcode)
		end)

		when('invoked incorrectly', function()
			local state

			before_each(function()
				state = execute('./target/release/prefab state')
			end)

			it('fails', function()
				assert.are_not.equal(0, state.exitcode)
			end)

			it('prints usage to stderr', function()
				assert.is_not_nil(string.find(state.stderr, 'USAGE'))
			end)
		end)
	end)

	describe('delete', function()
		it('suceeds', function()
			local state = execute('./target/release/prefab delete foo')
			assert.are.equal(0, state.exitcode)
		end)

		when('invoked incorrectly', function()
			local state

			before_each(function()
				state = execute('./target/release/prefab delete')
			end)

			it('fails', function()
				assert.are_not.equal(0, state.exitcode)
			end)

			it('prints usage to stderr', function()
				assert.is_not_nil(string.find(state.stderr, 'USAGE'))
			end)
		end)
	end)
end)

function read_file(path)
	local file = io.open(path, 'rb')
	if not file then return nil end
	local content = file:read('*a')
	file:close()
	return content
end

function execute(command)
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

function make_command(cmd)
	local stdout_path = os.tmpname()
	local stderr_path = os.tmpname()
	return cmd .. ' 1>' .. stdout_path .. ' 2>' .. stderr_path, stdout_path, stderr_path
end
