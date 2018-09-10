when = describe

describe('prefab', function()
	when('invoked with -h', function()
		local state

		before_each(function()
			state = execute('./target/release/prefab -h')
		end)

		it('succeeds', function()
			assert.is_true(state.success)
		end)

		it('prints help', function()
			assert.is_not_nil(string.find(state.output, "USAGE"))
		end)
	end)

	when('invoked with -V', function()
		local state

		before_each(function()
			state = execute('./target/release/prefab -V')
		end)

		it('succeeds', function()
			assert.is_true(state.success)
		end)

		it('prints the version', function()
			assert.are.equal("dev\n", state.output)
		end)
	end)

	when('invoked incorrectly', function()
		it('fails', function()
			local state = execute('./target/release/prefab')
			assert.is_false(state.success)
		end)
	end)

	describe('create', function()
		it('suceeds', function()
			local state = execute('./target/release/prefab create foo')
			assert.is_true(state.success)
		end)
	end)

	describe('state', function()
		it('suceeds', function()
			local state = execute('./target/release/prefab state foo')
			assert.is_true(state.success)
		end)
	end)

	describe('delete', function()
		it('suceeds', function()
			local state = execute('./target/release/prefab delete foo')
			assert.is_true(state.success)
		end)
	end)
end)

function execute(name)
	local handle = io.popen(name, 'r')
	local output = handle:read('*a')
	local exitstatus = {handle:close()}
	return {
		output=output,
		success=exitstatus[1] == true,
		exitcode=exitstatus[3],
	}
end
