cvl.on({"key.w", "key.up"}, function ()
    return cvl.action("camera.move.up")
end)

cvl.on({"key.a", "key.left"}, function ()
    return cvl.action("camera.move.left")
end)

cvl.on({"key.s", "key.down"}, function ()
    return cvl.action("camera.move.down")
end)

cvl.on({"key.d", "key.right"}, function ()
    return cvl.action("camera.move.right")
end)
