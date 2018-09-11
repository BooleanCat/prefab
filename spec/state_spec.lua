local utils = require 'spec.utils'

local when = describe

describe('state', function()
	it('suceeds', function()
		local state = utils.execute './target/release/prefab state foo'
		assert.are.equal(0, state.exitcode)
	end)

	when('invoked incorrectly', function()
		local state

		before_each(function()
			state = utils.execute './target/release/prefab state'
		end)

		it('fails', function()
			assert.are_not.equal(0, state.exitcode)
		end)

		it('prints usage to stderr', function()
			assert.is_not_nil(string.find(state.stderr, 'USAGE'))
		end)
	end)

	when('invoked with -h', function()
		local state

		before_each(function()
			state = utils.execute './target/release/prefab state -h'
		end)

		it('succeeds', function()
			assert.are.equal(0, state.exitcode)
		end)

		it('prints help', function()
			assert.is_not_nil(string.find(state.stdout, 'USAGE'))
		end)
	end)
end)