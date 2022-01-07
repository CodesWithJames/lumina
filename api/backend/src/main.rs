#![feature(never_type)] // This is needed to use the `!` type.

use async_std::net::{TcpListener, TcpStream};
use async_std::sync::Mutex;


use std::sync::mpsc::{SyncSender};
use std::sync::{Arc, mpsc};

#[derive(Debug)]
pub enum Code {
    CriticalError
}

#[derive(Debug)]
pub enum BackendError {
    BugError(Code, Box<str>)
}

impl BackendError {
    pub fn bug_as_result<M>(code: Code, msg: M) -> Result<!, Self>
    where M: Into<Box<str>> {
        Err(Self::bug(code, msg))
    }

    pub fn bug<M> (code: Code, msg: M) -> Self
    where M: Into<Box<str>> {
        Self::BugError(code, msg.into())
    }
}

pub struct LuminaServer {
    stop_sender: Mutex<Option<SyncSender<()>>>,
}

impl LuminaServer {

    /// Creates a new server instance.
    pub async fn new() -> Result<Self, BackendError> {
        let server = Self {
            stop_sender: Mutex::new(None),
        };

        Ok(server)
    }

    /// ## Starts the server
    /// Start a tcp server to respond to requests.
    /// - Throws an error if the server cannot be started.
    /// - Returns a future when the server has stopped listening for requests.
    pub async fn start(self: Arc<Self>) -> Result<(), BackendError> {
        // if the server is already running, return ok
        if self.stop_sender.lock().await.is_some() {
            return Ok(());
        }

        let listener = TcpListener::bind("0.0.0.0:80")
            .await
            .map_err(|_| BackendError::bug(Code::CriticalError, "Failed to bind to port"))?;

        // create a channel to send the stop signal
        // add the channel to the server, so we can keep track of it
        let (stop_sender, stop_receiver) = mpsc::sync_channel(1);
        {
            let mut stop_sender_lock = self.stop_sender.lock().await;
            *stop_sender_lock = Some(stop_sender); // stop sender lock dropped after this block executes
        }

        // start a task to accept connections
        let loop_task = async_std::task::spawn(async move {
            let listener = listener;
            while let Ok((stream, _)) = listener.accept().await {
                // start a new task to handle the connection
                async_std::task::spawn(async move {
                    let _ = handle_stream(stream).await;
                });
            }
        });

        // block until the stop signal is received
        stop_receiver.recv().unwrap();

        // cancel the loop task
        // this will cause the listener to stop accepting connections
        // but the existing connections will continue to run until they complete
        loop_task.cancel().await;

        Ok(())
    }

    /// ## Stops the server
    /// Stop the server by sending a stop signal to the server.
    /// - Returns a future that will complete when the server has stopped.
    pub async fn stop(self: &Arc<Self>) -> Result<(), BackendError> {
        let stop_sender = self.stop_sender.lock().await;
        if let Some(s) = &*stop_sender {
            s.send(()).map_err(
                |_| BackendError::bug(Code::CriticalError, "Failed to send stop signal"),
            )?;
        }
        Ok(())
    }
}

async fn handle_stream(stream: TcpStream) -> Result<(), BackendError> {
    stream.write();
    Ok(())
}

fn main() {
    // code that writes the mongo string to env file, synced up with devspace on connection
    // tests should be run against the mongo string in the env file
    // a better way to do this would be to use a docker container to run the tests
    // but this might reduce debug functionality for the developer
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[async_std::test]
    async fn server_starts_and_stops() -> Result<(), BackendError> {
        let server = Arc::new(LuminaServer::new().await?);

        let start_future = server.clone().start();

        let stop_future = async_std::task::spawn(async move {
            async_std::task::sleep(std::time::Duration::from_secs(1)).await;
            let _ = server.stop().await;
        });

        let _ = futures::join!(start_future, stop_future);

        Ok(())
    }
}