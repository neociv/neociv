(fn actbk! [action]
  `(fn []
     (cvl.dispatch ,(tostring action))))

(fn op! [op]
  `(fn []
      (cvl.op ,(tostring op))))

{
  : actbk!
  : op!
}

