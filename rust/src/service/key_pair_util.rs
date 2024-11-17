use bip39::{Language, Mnemonic, MnemonicType};
use libp2p::PeerId;
use libp2p::identity::Keypair;

pub fn generate_new_key_pair() -> Keypair {
    let local_keypair: Keypair = Keypair::generate_ed25519();
    local_keypair
}
fn generate_new_mnemonic() -> Mnemonic {
    let mnemonic = Mnemonic::new(MnemonicType::Words15, Default::default());
    println!("{}", mnemonic);
    mnemonic
}
pub fn get_mnemonic_from_str(mnemonic_string: &str) -> Mnemonic {
    Mnemonic::from_phrase(mnemonic_string, Language::English).unwrap()
}
pub fn get_keypair_from_mnemonic_str(mnemonic_string: &str) -> Keypair {
    let mnemonic = Mnemonic::from_phrase(mnemonic_string, Default::default()).unwrap();
    get_keypair_from_mnemonic(mnemonic)
}
fn get_keypair_from_mnemonic(mnemonic: Mnemonic) -> Keypair {
    let mut entropy_bytes = [0u8; 32];
    entropy_bytes[..mnemonic.entropy().len()].copy_from_slice(&mnemonic.entropy());
    Keypair::ed25519_from_bytes(entropy_bytes).unwrap()
}
pub fn get_peer_id(keypair: &Keypair) -> PeerId {
    let local_peer_id: PeerId = PeerId::from(keypair.public());
    local_peer_id
}

#[cfg(test)]
mod tests {
    use bip39::Mnemonic;
    use super::*;

    #[test]
    fn test_restoring_keypair() {
        let mnemonic = generate_new_mnemonic();
        let mnemonic_string = mnemonic.to_string();
        let keypair = get_keypair_from_mnemonic(mnemonic);
        let restored_keypair = get_keypair_from_mnemonic_str(&mnemonic_string);
        assert_eq!(keypair.public(), restored_keypair.public());
    }
    #[test]
    fn test_mnemonic_to_keypair() {
        let mnemonic_string = "actress barely uniform warm task parade hamster brave patient airport inform seek cigar shop try";
        let mnemonic = Mnemonic::from_phrase(mnemonic_string, Default::default()).unwrap();
        let keypair = get_keypair_from_mnemonic(mnemonic);
        let peer_id = keypair.public().to_peer_id();
        assert_eq!(peer_id.to_string(), "12D3KooWLvqSPsvygzZpUGW9NucmwZZVkhEQhn5Wz9hv5tRrEzhh");
    }
}
