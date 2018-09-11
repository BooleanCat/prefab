local utils = require 'spec.utils'

local when = describe

describe('prefab', function()
	when('invoked with -h', function()
		local state

		before_each(function()
			state = utils.execute './target/release/prefab -h'
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
			state = utils.execute './target/release/prefab -V'
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
			state = utils.execute './target/release/prefab'
		end)

		it('fails', function()
			assert.are_not.equal(0, state.exitcode)
		end)

		it('prints usage to stderr', function()
			assert.is_not_nil(string.find(state.stderr, 'USAGE'))
		end)
	end)
end)

