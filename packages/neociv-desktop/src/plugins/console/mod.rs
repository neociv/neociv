use std::io::{self, BufRead, Write};

use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
use crossbeam::channel::{bounded, Receiver};

use neociv_civil::runtime::{repl::NeocivRepl, NeocivRuntime};

#[derive(Component, Resource)]
pub struct REPLChannel {
    receiver: Receiver<String>,
}

fn repl_input(cvl: ResMut<NeocivRuntime>, channel: Res<REPLChannel>) {
    if let Ok(line) = channel.receiver.try_recv() {
        let result = cvl.lua_repl_line(&line);
        match result {
            Ok(s) => println!("{}", s),
            Err(ex) => error!("{:?}", ex),
        }
    }
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
    commands.insert_resource(REPLChannel { receiver: rx });
}

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_io_thread);
        app.add_system(repl_input);
    }
}
