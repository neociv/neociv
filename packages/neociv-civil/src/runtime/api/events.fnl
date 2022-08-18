;; Manage camera updates
(cvl.on :camera.move.up (fn [evt]
    (cvl.set_state :camera.position.z (+ (cvl.get_state :camera.position.z) (cvl.get_config :camera.speed)))
))

