;; Manage camera updates
(cvl.on :camera.move.up
        (fn [evt]
          (cvl.set :camera.position.z
                   (+ (cvl.get :camera.position.z)
                      (cvl.get_config :camera.speed)))))

(cvl.on :camera.move.down
        (fn [evt]
          (cvl.set :camera.position.z
                   (- (cvl.get :camera.position.z)
                      (cvl.get_config :camera.speed)))))

(cvl.on :camera.move.left
        (fn [evt]
          (cvl.set :camera.position.z
                   (- (cvl.get :camera.position.x)
                      (cvl.get_config :camera.speed)))))

(cvl.on :camera.move.right
        (fn [evt]
          (cvl.set :camera.position.z
                   (+ (cvl.get :camera.position.x)
                      (cvl.get_config :camera.speed)))))

