use std::collections::HashSet;
use cfx_types::U256;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use parking_lot::Mutex;

/// The verification message kinds that transmit between ConsensusGraph
/// and BlockVerifier.
pub enum VerificationMsg {
    /// NewBlock is the message that ConsensusGraph sends to BlockVerifier
    /// to notify the process state of a new block. "pending" is the most
    /// important field. If pending is true, it means that ConsensusGraph
    /// expects BlockVerifier to perform verification on its validity,
    /// stability, and adaptivity (those three fields are ignored in this
    /// case).
    NewBlock {
        me: usize,
        parent: usize,
        referees: Vec<usize>,
        past_weight_lower: U256,
        past_weight_upper: U256,
        pending: bool,
        valid: bool,
        stable: bool,
        adaptive: bool,
    },
    /// WaitVerify is the message that ConsensusGraph sends to BlockVerifier
    /// to indicate that ConsensusGraph needs the verification results of
    /// the specific block to proceed. BlockVerifier should prioritize this
    /// block verification.
    WaitVerify(usize),
    /// VerificationResults is the message that BlockVerifier sends to
    /// ConsensusGraph to indicate the verification results of a pending
    /// block.
    VerificationResults {
        me: usize,
        valid: bool,
        stable: bool,
        adaptive: bool,
    },
}

pub struct BlockVerifier {
    block_verifier_to_consensus_sender: Mutex<Sender<VerificationMsg>>,
    consensus_to_block_verifier_receiver: Mutex<Receiver<VerificationMsg>>,
}

impl BlockVerifier {
    pub fn new(block_verifier_to_consensus_sender: Sender<VerificationMsg>,
               consensus_to_block_verifier_receiver: Receiver<VerificationMsg>) -> Self {
        Self {
            block_verifier_to_consensus_sender: Mutex::new(block_verifier_to_consensus_sender),
            consensus_to_block_verifier_receiver: Mutex::new(consensus_to_block_verifier_receiver),
        }
    }

    pub fn start(&self) {
        thread::Builder::new()
            .name("Block Verifier".into())
            .spawn( move || loop {
                match self.consensus_to_block_verifier_receiver.lock().recv() {
                    Ok(VerificationMsg::NewBlock {
                        me,
                        parent,
                        referees,
                        past_weight_lower,
                        past_weight_upper,
                        pending,
                        valid,
                        stable,
                        adaptive,
                       }) => {
                        // TODO
                    },
                    Ok(VerificationMsg::WaitVerify(to_verify_index)) => {
                        // TODO
                    },
                    Ok(msg) => panic!("Unexpected message type."),
                    Err(_) => break,
                }
            }).expect("Block Verifier thread failed!");
    }
}
