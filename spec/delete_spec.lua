local exec = require('spec.exec')

local when = describe

describe('delete', function()
	it('suceeds', function()
		local exitcode = exec.command('./target/release/prefab delete foo')
		assert.are.equal(0, exitcode)
	end)

	when('invoked incorrectly', function()
		local exitcode, stderr

		before_each(function()
			exitcode, _, stderr = exec.command('./target/release/prefab delete')
		end)

		it('fails', function()
			assert.are_not.equal(0, exitcode)
		end)

		it('prints usage to stderr', function()
			assert.is_not_nil(string.find(stderr, 'USAGE'))
		end)
	end)

	when('invoked with -h', function()
		local exitcode, stdout

		before_each(function()
			exitcode, stdout = exec.command('./target/release/prefab delete -h')
		end)

		it('succeeds', function()
			assert.are.equal(0, exitcode)
		end)

		it('prints help', function()
			assert.is_not_nil(string.find(stdout, 'USAGE'))
		end)
	end)
end)
