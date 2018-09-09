describe('prefab', function()
	it('does nothing', function()
		local output = execute('./target/release/prefab', 'r')
		assert.are.equal(output, 'Hello, world!\n')
	end)
end)

function execute(name)
	local handle = io.popen(name, 'r')
	local output = handle:read('*a')
	local success, _ = handle:close()
	assert.is_true(success)
	return output
end
