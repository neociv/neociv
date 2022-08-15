;; Convenient action dispatcher callback
(fn actbk! [action]
    `(> (fn [] (cvl.dispatch ,(tostring action)))))

{
: actbk!
}