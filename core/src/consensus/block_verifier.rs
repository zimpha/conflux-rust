use cfx_types::U256;
use std::{
    sync::mpsc::{Receiver, Sender},
    thread,
};

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
    block_verifier_to_consensus_sender: Sender<VerificationMsg>,
}

impl BlockVerifier {
    pub fn new(
        block_verifier_to_consensus_sender: Sender<VerificationMsg>,
    ) -> Self {
        Self {
            block_verifier_to_consensus_sender,
        }
    }

    pub fn start(
        self, consensus_to_block_verifier_receiver: Receiver<VerificationMsg>,
    ) {
        thread::Builder::new()
            .name("Block Verifier".into())
            .spawn(move || {
                loop {
                    match consensus_to_block_verifier_receiver.recv() {
                        Ok(VerificationMsg::NewBlock {
                            me,
                            parent: _,
                            referees: _,
                            past_weight_lower: _,
                            past_weight_upper: _,
                            pending: _,
                            valid,
                            stable,
                            adaptive,
                        }) => {
                            // TODO, This is just a dummy place holder
                            if me == 0 {
                                self.block_verifier_to_consensus_sender
                                    .send(
                                        VerificationMsg::VerificationResults {
                                            me,
                                            valid,
                                            stable,
                                            adaptive,
                                        },
                                    )
                                    .expect("cannot fail");
                            }
                        }
                        Ok(VerificationMsg::WaitVerify(_to_verify_index)) => {
                            // TODO
                        }
                        Ok(_) => panic!("Unexpected message type."),
                        Err(_) => break,
                    }
                }
                info!("Block Verifier quit!");
            })
            .expect("Block Verifier thread failed!");
    }
}
