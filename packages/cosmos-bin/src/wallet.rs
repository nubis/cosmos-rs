use anyhow::Result;
use cosmos::{AddressHrp, RawAddress, SeedPhrase};

use crate::gen_wallet;

#[derive(clap::Parser)]
pub(crate) struct Opt {
    #[clap(subcommand)]
    sub: Subcommand,
}

#[derive(clap::Parser)]
enum Subcommand {
    /// Generate wallet
    GenWallet {
        /// Address type, supports any valid Human Readable Part like cosmos, osmo, or juno
        address_type: AddressHrp,
    },
    /// Print the address for the given phrase
    PrintAddress {
        /// HRP (human readable part) of the address, e.g. osmo, inj
        hrp: AddressHrp,
        /// Phrase
        phrase: SeedPhrase,
    },
    /// Print the address for a different chain
    ChangeAddressType {
        /// Original address
        orig: RawAddress,
        /// Destination address HRP (human-readable part)
        hrp: AddressHrp,
    },
}

pub(crate) async fn go(Opt { sub }: Opt) -> Result<()> {
    match sub {
        Subcommand::GenWallet { address_type } => gen_wallet(address_type)?,
        Subcommand::PrintAddress { hrp, phrase } => {
            println!("{}", phrase.with_hrp(hrp)?);
        }
        Subcommand::ChangeAddressType {
            orig,
            hrp: address_type,
        } => {
            println!("{}", orig.with_hrp(address_type));
        }
    }
    Ok(())
}
