use std::convert::TryFrom;
use std::str::FromStr;

use nng::{Aio, Socket};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};
use uuid::Uuid;

use super::error::{Error, Result};
use super::fbs::{pkgs_to_fbs, shared_ctx_to_fbs};
use crate::gen;
use crate::proto::ExperimentID;
use crate::types::WorkerIndex;
use crate::worker::runner::comms::{outbound::OutboundFromRunnerMsg, ExperimentInitRunnerMsg};

fn experiment_init_to_nng(init: &ExperimentInitRunnerMsg) -> Result<nng::Message> {
    // TODO - initial buffer size
    let mut fbb = flatbuffers::FlatBufferBuilder::new();
    let experiment_id =
        gen::init_generated::ExperimentID(*(Uuid::from_str(&init.experiment_id)?.as_bytes()));

    // Build the SharedContext Flatbuffer Batch objects and collect their offsets in a vec
    let shared_context = shared_ctx_to_fbs(&mut fbb, &init.shared_context);

    // Build the Flatbuffer Package objects and collect their offsets in a vec
    let package_config = pkgs_to_fbs(&mut fbb, &init.package_config)?;
    let msg = gen::init_generated::Init::create(
        &mut fbb,
        &crate::gen::init_generated::InitArgs {
            experiment_id: Some(&experiment_id),
            worker_index: init.worker_index as u64,
            shared_context: Some(shared_context),
            package_config: Some(package_config),
        },
    );

    fbb.finish(msg, None);
    let bytes = fbb.finished_data();

    let mut nanomsg = nng::Message::with_capacity(bytes.len());
    nanomsg.push_front(bytes);

    Ok(nanomsg)
}

/// Only used for receiving messages from the Python process,
/// except for the init message, which is sent once in response
/// to an init message request
pub struct NngReceiver {
    route: String,

    // Used in the aio to receive nng messages from the Python process.
    from_py: Socket,
    aio: Aio,

    // Sends the results of operations (i.e. results of trying to
    // receive nng messages) in the aio.
    // aio_result_sender: UnboundedSender<nng::Message>,

    // Receives the results of operations from the aio.
    aio_result_receiver: UnboundedReceiver<nng::Message>,
}

impl NngReceiver {
    pub fn new(experiment_id: ExperimentID, worker_index: WorkerIndex) -> Result<Self> {
        let route = format!("ipc://{}-frompy{}", experiment_id, worker_index);
        let from_py = Socket::new(nng::Protocol::Pair0)?;

        let (aio_result_sender, aio_result_receiver) = unbounded_channel();
        let aio = Aio::new(move |_aio, res| match res {
            nng::AioResult::Recv(Ok(m)) => {
                aio_result_sender.send(m).expect("Should be able to send");
            }
            nng::AioResult::Sleep(Ok(_)) => {}
            nng::AioResult::Send(_) => {
                log::warn!("Unexpected send result");
            }
            nng::AioResult::Recv(Err(nng::Error::Canceled)) => {}
            nng::AioResult::Recv(Err(nng::Error::Closed)) => {}
            _ => panic!("Error in the AIO, {:?}", res),
        })?;

        Ok(Self {
            route,
            from_py,
            aio,
            aio_result_receiver,
        })
    }

    pub fn init(&self, init_msg: &ExperimentInitRunnerMsg) -> Result<()> {
        self.from_py.listen(&self.route)?;

        // let listener = nng::Listener::new(&self.from_py, &self.route)?;
        let _init_request = self.from_py.recv()?;
        self.from_py // Only case where `from_py` is used for sending
            .send(experiment_init_to_nng(init_msg)?)
            .map_err(|(msg, err)| Error::NngSend(msg, err))?;

        let _init_ack = self.from_py.recv()?;
        // listener.close();
        Ok(())
    }

    pub async fn get_recv_result(&mut self) -> Result<OutboundFromRunnerMsg> {
        let nng_msg = self
            .aio_result_receiver
            .recv()
            .await
            .ok_or(Error::OutboundReceive)?;

        self.from_py.recv_async(&self.aio)?;
        Ok(OutboundFromRunnerMsg::try_from(nng_msg).map_err(|err| {
            Error::from(format!(
                "Failed to convert nng message to OutboundFromRunnerMsg: {}",
                err.to_string()
            ))
        })?)
    }
}

impl Drop for NngReceiver {
    fn drop(&mut self) {
        // TODO: Check whether nng already does this when a socket is dropped
        self.from_py.close();
    }
}
