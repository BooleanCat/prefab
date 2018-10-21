local utils = require('pl.utils')
local path = require('pl.path')
local posix = require('posix')

local prefab_init = path.join('.', 'target', 'release', 'prefab-init')

describe('prefab-init', function()
	it('is long running', function()
		local pid = posix.fork()
		if pid == 0 then
			posix.unistd.exec(prefab_init, {})
		end

		local process_name = function()
			local _, _, stdout = utils.executeex('ps -p ' .. pid .. ' -o comm=')
			return stdout
		end

		eventually_equal(process_name, 'prefab-init\n', 1)

		-- TODO: check it continues to live for a bit

		-- TODO: wait for it to die
		local _, exitcode = utils.executeex('kill ' .. pid)
		assert.are.equal(0, exitcode)
	end)
end)

-- TODO: formalise as `eventually(f).should.return(a)`
function eventually_equal(f, a, threshold)
	local elapsed = 0
	while true do
		if f() == a then
			return
		end

		posix.unistd.sleep(1)
		elapsed = elapsed + 1

		if elapsed >= threshold then
			assert.are.equal(f(), a)
		end
	end
end

-- TODO: implement `consistently(f).should.return(a)`
