(fn actbk [action]
    `(fn [] (print ,(tostring action))))

(set package.preload["cvl.macros"] { : actbk })