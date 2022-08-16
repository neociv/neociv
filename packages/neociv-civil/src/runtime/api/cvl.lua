cvl = cvl or {
    events = {}
}

function cvl.on (event, handler)
    events = type(event) == "table" and event or { event }
    for _, evt in ipairs(events) do
        if cvl.events[evt] == nil then
            cvl.events[evt] = {}
        end
        table.insert(cvl.events[evt], handler)
    end
    return cvl
end

function cvl.dispatch (event, data)
    if cvl.events[event] ~= nil then
        table.foreach(cvl.events[evt], function (handler)
            handler({ type = event, data = data })
        end)
    end
    return cvl
end

return cvl