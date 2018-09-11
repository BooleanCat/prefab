local utils = require('pl.utils')

local when = describe

describe('prefab', function()
	when('invoked with -h', function()
		local exitcode, stdout

		before_each(function()
			_, exitcode, stdout = utils.executeex('./target/release/prefab -h')
		end)

		it('succeeds', function()
			assert.are.equal(0, exitcode)
		end)

		it('prints help', function()
			assert.is_not_nil(string.find(stdout, 'USAGE'))
		end)
	end)

	when('invoked with -V', function()
		local exitcode, stdout

		before_each(function()
			_, exitcode, stdout = utils.executeex('./target/release/prefab -V')
		end)

		it('succeeds', function()
			assert.are.equal(0, exitcode)
		end)

		it('prints the version', function()
			assert.are.equal("prefab 0.1.0\n", stdout)
		end)
	end)

	when('invoked incorrectly', function()
		local exitcode, stderr

		before_each(function()
			_, exitcode, _, stderr = utils.executeex('./target/release/prefab')
		end)

		it('fails', function()
			assert.are_not.equal(0, exitcode)
		end)

		it('prints usage to stderr', function()
			assert.is_not_nil(string.find(stderr, 'USAGE'))
		end)
	end)
end)

