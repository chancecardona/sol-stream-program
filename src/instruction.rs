use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

/// Instructions supported by the sol-streaming program.
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum StreamInstruction {
    /// Create a stream with a escrow account created and funded by sender
    /// account should have a total_lamport=admin_cut+program_rent_account+amount_to_send.
    ///
    /// Accounts expected:
    ///
    /// `[writable]` escrow account, it will hold all necessary info about the trade.
    /// `[signer]` sender account
    /// `[]` receiver account
    /// `[]` Admin account
    CreateStream,

    /// Withdraw from a stream for receiver
    ///
    /// Accounts expected:
    ///
    /// `[writable]` escrow account, it will hold all necessary info about the trade.
    /// `[signer]` receiver account
    WithdrawFromStream,

    /// Close a stream and transfer tokens between sender and receiver.
    ///
    /// Accounts expected:
    ///
    /// `[writable]` escrow account, it will hold all necessary info about the trade.
    /// `[signer]` sender account
    /// `[]` receiver account
    CloseStream,
}
