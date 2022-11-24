cvl.on({ "key.w", "key.up" }, function()
    return cvl.dispatch("camera.move.up")
end)

cvl.on({ "key.a", "key.left" }, function()
    return cvl.dispatch("camera.move.left")
end)

cvl.on({ "key.s", "key.down" }, function()
    return cvl.dispatch("camera.move.down")
end)

cvl.on({ "key.d", "key.right" }, function()
    return cvl.dispatch("camera.move.right")
end)
