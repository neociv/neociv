cvl = cvl or {
    revision = 0,
    events = {},
    config = {},
    state = nil
}

-- Generate a chain table from a string or safely pass through a table
local function chain(input)
    if type(event) == "table" then
        return input
    else
        return ""
    end
end

function cvl.inject_state(state)
    cvl.state = state
    --if cvl.state.revision ~= cvl.revision then
    --    cvl.revision = cvl.state.revision
    --    cvl.dispatch("state.updated", cvl.state)
    --end
end

-- Listen for events
function cvl.on(event, handler)
    local events = type(event) == "table" and event or { event }
    for _, evt in ipairs(events) do
        if cvl.events[evt] == nil then
            cvl.events[evt] = {}
        end
        table.insert(cvl.events[evt], handler)
    end
    return cvl
end

function cvl.dispatch(event, data)
    if cvl.events[event] ~= nil then
        for _, handler in pairs(cvl.events[event]) do
            handler({ type = event, data = data })
        end
    end
    return cvl
end

-- Set a property in the state, will dispatch a new state
function cvl.set(prop_path, value)
    cvl.state = cvl.state;
end

-- Get a property in the state
function cvl.get(prop_path)
    return cvl.state:get(prop_path)
end

-- Get a config property or the default value
function cvl.get_config(prop_path, default)
    return nil
end

package.preload["cvl"] = function() return cvl end

return cvl
