use std::io::{self, BufRead, Write};

use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
use crossbeam::channel::{bounded, Receiver};

fn repl_input(channel: Res<Receiver<String>>) {
}

fn spawn_io_thread(mut commands: Commands) {
    let thread_pool = AsyncComputeTaskPool::get();
    info!("Starting console...");
    io::stdout().flush().unwrap();

    let (tx, rx) = bounded(1);
    let task = thread_pool.spawn(async move {
        let stdin = io::stdin();
        loop {
            let line = stdin.lock().lines().next().unwrap().unwrap();
            tx.send(line)
                .expect("error sending user input to other thread");
        }
    });
    task.detach();
    commands.insert_resource(rx);
}

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_io_thread);
        app.add_system(repl_input);
    }
}
