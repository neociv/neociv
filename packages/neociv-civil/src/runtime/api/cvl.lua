cvl = cvl or {
    events = {}
}

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

package.preload["cvl"] = function() return cvl end

return cvl
