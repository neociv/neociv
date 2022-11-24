;; Manage camera updates
(cvl.on :camera.move.up
        (fn [evt]
          (cvl.op :mod.camera.position { :y 10 })))

(cvl.on :camera.move.down
        (fn [evt]
          (cvl.op :mod.camera.position { :y -10 })))

(cvl.on :camera.move.left
        (fn [evt]
          (cvl.op :mod.camera.position { :x -10 })))

(cvl.on :camera.move.right
        (fn [evt]
          (cvl.op :mod.camera.position { :x 10 })))

