local inspect = require("inspect")

cvl = cvl or {
    revision = -1,
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

local function strsplit(str, sep)
    sep = sep or "%s";
    local tbl = {};
    local i = 1;
    for str in string.gmatch(str, "([^"..sep.."]+)") do
        tbl[i] = str;
        i = i + 1;
    end
    return tbl;
end

local function setter(tbl, path, val)
    local value;
    local keys = strsplit(path, ".");
    local pre = { table.unpack(keys, 1, #keys - 1) }
    local final = table.unpack(keys, #keys)

    for _, key in pairs(pre) do
        value = (value and value[key]) or tbl[key];
    end

    value[final] = val

    -- TODO: Update the revision upon successful setting
    tbl.revision = tbl.revision + 1
    
    return tbl
end

function cvl.inject_state(state)
    if cvl.state == nil or state ~= nil and state.revision ~= cvl.revision  then
        cvl.state = state
        cvl.revision = cvl.state.revision
        cvl.dispatch("state.updated", cvl.state)
    end
end

-- Push the changed state back up to the runtime
function cvl.push_state(state)
    return cvl
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

-- Dispatch an event to handlers
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
    local state = setter(cvl.state, prop_path, value)
    cvl.inject_state(state)
    return cvl
end

-- Get a property in the state
function cvl.get(prop_path)
    return cvl.state[prop_path]
end

-- Get a config property or the default value
function cvl.get_config(prop_path, default)
    return default or nil
end

-- Tells cvl to go and load a package directory
function cvl.load_pkg(pkg_path)
    return cvl
end

-- Generic register
function cvl.register(type, id, data)
    return cvl
end

function cvl.inspect(o)
    print(inspect(o))
end

function cvl.inspect_state()
    cvl.inspect(cvl.state)
end

function cvl.op(action, args)
    cvl.inject_state(cvl._engine_do(action, args))
    return cvl
end

package.preload["cvl"] = function() return cvl end

return cvl
