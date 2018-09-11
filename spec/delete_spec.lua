local utils = require('pl.utils')
local path = require('pl.path')

local when = describe
local prefab = path.join('.', 'target', 'release', 'prefab')

describe('delete', function()
	it('suceeds', function()
		local _, exitcode = utils.executeex(prefab .. ' delete foo')
		assert.are.equal(0, exitcode)
	end)

	when('invoked incorrectly', function()
		local exitcode, stderr

		before_each(function()
			_, exitcode, _, stderr = utils.executeex(prefab .. ' delete')
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
			_, exitcode, stdout = utils.executeex(prefab .. ' delete -h')
		end)

		it('succeeds', function()
			assert.are.equal(0, exitcode)
		end)

		it('prints help', function()
			assert.is_not_nil(string.find(stdout, 'USAGE'))
		end)
	end)
end)
