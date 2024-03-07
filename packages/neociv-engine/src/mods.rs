/// Mods are effectively an action that specifies something like either a target and a value
/// or a custom definition

/// The type of mod for nodes comes from something like the following then as a node's metadata should be thus
/// vars: { name: String, value: enum { String, Float, Integer, Boolean } }
/// The values of these could be changed even!

/// So then modifications can thus be described as roughly
/// { type: "mod.value", target: "player.balance.gold", action: "add", value: 100 }

/// But what about turns - there are some reliable parts of the stages of a game that it would be good to have
/// { type: "mod.value", when: "endofturn", repeat: 30, target: "player.balance.gold", action: "add", value: 100 }
/// 
/// Custom modifications could then be explained, those that call back to functions for example could use custom text
/// { type: "mod.value", when: "endofturn", repeat: 3, target: "$function.id", text: "Do something special" }
/// 
/// They should also be able to take vars in the case of functions? These could then be passed to functions.
/// For example it would be good if *all* modifications made by tree nodes could attach their vars automatically
/// { type: "mod.custom", when: "endofturn", repeat: Infinity, target: "$function.id", text: "Do something special, via {{x}}", vars: vec![{ name: "x", value: 42.0 }] }
/// And then what - this function could return basic "mod.value" and if it does that then they can be described! The function can provide its custom text *and* we can list
/// the describable mods it returns provided there are no side effects! So side effect types could be mod.custom, and mod.custom.strict could be for functions without side effects
/// but that return a list!
/// 
/// They also need to have ids - the function to create them should return the id or one can be provided but hard error on conflicts. The ids can then be used to remove them
/// or even force their prevention (?) - regardless just good practice. This will help with summarising data in many places of the UI.
/// These can just be a Vec on each civ's state struct.
/// *nice*
/// 
/// Should they also have a source descriptor?
/// eg. { ...action, from: "trade.route.id" }
/// Maybe a "category" so that all "trade", for example, can be grouped together, while still maintaining a *specific* source?