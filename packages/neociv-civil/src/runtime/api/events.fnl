;; Manage camera updates
(cvl.on :camera.move.up (fn [evt]
    (cvl.set :camera.position.z (+ (cvl.get :camera.position.z) (cvl.get_config :camera.speed)))
))

