-- Always on lua >= 5.4 so only package.searchers needed
table.insert(package.searchers, require("fennel.specials")["make-searcher"]())
